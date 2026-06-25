use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{QImage, QImageFormat, QString};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qimage.h");
        type QImage = cxx_qt_lib::QImage;
    }

    extern "RustQt" {
        #[qobject]
        #[namespace = "render_thread"]
        type RenderThread = super::RenderThreadRust;

        #[qsignal]
        #[cxx_name = "renderedImage"]
        fn rendered_image(self: Pin<&mut RenderThread>, image: &QImage, scale_factor: f64);

        fn render(
            self: Pin<&mut RenderThread>,
            center_x: f64,
            center_y: f64,
            scale_factor: f64,
            result_width: i32,
            result_height: i32,
            device_pixel_ratio: f64,
        );
    }

    impl cxx_qt::Threading for RenderThread {}
}

static NUM_PASSES: i32 = 8;
static COLOR_MAP_SIZE: usize = 512;

#[derive(Default)]
struct RenderInfo {
    center_x: f64,
    center_y: f64,
    scale_factor: f64,
    device_pixel_ratio: f64,
    result_width: i32,
    result_height: i32,
}

/// The Rust struct for the QObject
pub struct RenderThreadRust {
    render_info: Arc<Mutex<RenderInfo>>,
    color_map: Arc<Mutex<[u32; COLOR_MAP_SIZE]>>,
    restart: Arc<AtomicBool>,
    abort: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Default for RenderThreadRust {
    fn default() -> Self {
        let mut color_map = [0; COLOR_MAP_SIZE];
        for (i, color) in &mut color_map.iter_mut().enumerate() {
            *color = Self::rbg_from_wave_length(380. + (i as f64 * 400. / COLOR_MAP_SIZE as f64))
        }

        Self {
            render_info: Default::default(),
            color_map: Arc::new(Mutex::new(color_map)),
            restart: Default::default(),
            abort: Default::default(),
            thread: None,
        }
    }
}

impl RenderThreadRust {
    pub fn rbg_from_wave_length(wave: f64) -> u32 {
        let mut r = 0.;
        let mut g = 0.;
        let mut b = 0.;

        if wave >= 380. && wave <= 440. {
            r = -1. * (wave - 440.) / (440. - 380.);
            b = 1.;
        } else if wave >= 440. && wave <= 490. {
            g = (wave - 440.) / (490. - 440.);
            b = 1.;
        } else if wave >= 490. && wave <= 510. {
            g = 1.;
            b = -1. * (wave - 510.) / (510. - 490.);
        } else if wave >= 510. && wave <= 580. {
            r = (wave - 510.) / (580. - 510.);
            g = 1.;
        } else if wave >= 580. && wave <= 645. {
            r = 1.;
            g = -1. * (wave - 645.) / (645. - 580.);
        } else if wave >= 645. && wave <= 780. {
            r = 1.;
        }

        let s = if wave > 700. {
            0.3 + 0.7 * (780. - wave) / (780. - 700.)
        } else if wave < 420. {
            0.3 + 0.7 * (wave - 380.) / (420. - 380.)
        } else {
            1.
        };

        r = (r * s).powf(0.8);
        g = (g * s).powf(0.8);
        b = (b * s).powf(0.8);

        // Replicate a QRgb
        0xFF000000 | ((r as u32 * 255) << 16) | ((g as u32 * 255) << 8) | (b as u32 * 255)
    }
}

impl qobject::RenderThread {
    pub fn render(
        self: Pin<&mut qobject::RenderThread>,
        center_x: f64,
        center_y: f64,
        scale_factor: f64,
        result_width: i32,
        result_height: i32,
        device_pixel_ratio: f64,
    ) {
        // Drop lock after this
        {
            let mut render_info = self
                .render_info
                .lock()
                .unwrap_or_else(|error| error.into_inner());

            render_info.center_x = center_x;
            render_info.center_y = center_y;
            render_info.scale_factor = scale_factor;
            render_info.device_pixel_ratio = device_pixel_ratio;
            render_info.result_width = result_width;
            render_info.result_height = result_height;
        }

        if let Some(thread) = &self.thread {
            self.restart.store(true, Ordering::SeqCst);
            thread.thread().unpark();
        } else {
            self.run();
        }
    }

    fn run(self: Pin<&mut qobject::RenderThread>) {
        let render_info = self.render_info.clone();
        let color_map = self.color_map.clone();
        let restart = self.restart.clone();
        let abort = self.abort.clone();
        let qt_thread = self.qt_thread();

        self.rust_mut().thread = Some(thread::spawn(move || {
            loop {
                let center_x;
                let center_y;
                let scale_factor;
                let device_pixel_ratio;
                let result_width;
                let result_height;

                {
                    let render_info = render_info
                        .lock()
                        .unwrap_or_else(|error| error.into_inner());

                    center_x = render_info.center_x;
                    center_y = render_info.center_y;
                    scale_factor = render_info.scale_factor;
                    device_pixel_ratio = render_info.device_pixel_ratio;
                    result_width = render_info.result_width;
                    result_height = render_info.result_height;
                }

                let half_width = result_width / 2;
                let half_height = result_height / 2;
                let mut image = QImage::from_width_height_and_format(
                    result_width,
                    result_height,
                    QImageFormat::Format_RGB32,
                );

                image.set_device_pixel_ratio(device_pixel_ratio.into());

                let mut pass = 0;
                while pass < NUM_PASSES {
                    let max_iterations = (1 << (2 * pass + 6)) + 1;
                    let limit = 4;
                    let mut all_black = true;

                    for y in -half_height..half_height {
                        if restart.load(Ordering::SeqCst) {
                            break;
                        }

                        if abort.load(Ordering::SeqCst) {
                            return;
                        }

                        let scan_line = image.scan_line(y + half_height).cast::<u32>();
                        let ay = center_y + y as f64 * scale_factor;

                        for x in -half_width..half_width {
                            let ax = center_x + x as f64 * scale_factor;
                            let mut a1 = ax;
                            let mut b1 = ay;
                            let mut num_iterations = 0;

                            // do..while equivalent
                            loop {
                                num_iterations += 1;
                                let a2 = a1 * a1 - b1 * b1 + ax;
                                let b2 = 2. * a1 * b1 + ay;
                                if (a2 * a2 + b2 * b2) > limit as f64 {
                                    break;
                                }

                                num_iterations += 1;
                                a1 = a2 * a1 - b2 * b2 + ax;
                                b1 = 2. * a2 * b2 + ay;
                                if (a2 * a2 + b2 * b2) > limit as f64 {
                                    break;
                                }

                                if num_iterations >= max_iterations {
                                    break;
                                }
                            }

                            if num_iterations < max_iterations {
                                let color_map =
                                    color_map.lock().unwrap_or_else(|error| error.into_inner());

                                unsafe {
                                    *scan_line.add(1) = color_map[num_iterations % color_map.len()];
                                }

                                all_black = false;
                            } else {
                                unsafe {
                                    *scan_line.add(1) = 0xFF000000;
                                }
                            }
                        }
                    }

                    if all_black && pass == 0 {
                        pass = 4;
                    } else {
                        if !restart.load(Ordering::SeqCst) {
                            let message = format!(
                                "Pass {} / {NUM_PASSES}, max iterations: {max_iterations}",
                                pass + 1
                            );

                            image.set_text(&QString::from("info"), &QString::from(&message));
                            let image = image.clone();

                            let _ = qt_thread.queue(move |render_thread| {
                                render_thread.rendered_image(&image, scale_factor);
                            });
                        }

                        pass += 1;
                    }
                }

                if !restart.load(Ordering::SeqCst) {
                    thread::park();
                }

                restart.store(false, Ordering::SeqCst);
            }
        }));
    }
}
