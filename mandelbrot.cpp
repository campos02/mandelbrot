#include "mandelbrot.h"

#include <QGesture>
#include <QGestureEvent>
#include <QKeyEvent>
#include <QPainter>

#include <mandelbrot_rust/src/render_thread.cxxqt.h>
#include <math.h>

constexpr double DefaultCenterX = -0.637011;
constexpr double DefaultCenterY = -0.0395159;
constexpr double DefaultScale = 0.00403897;

constexpr double ZoomInFactor = 0.8;
constexpr double ZoomOutFactor = 1 / ZoomInFactor;
constexpr int ScrollStep = 20;

Mandelbrot::Mandelbrot(QWidget *parent)
    : QWidget(parent)
    , centerX(DefaultCenterX)
    , centerY(DefaultCenterY)
    , pixmapScale(DefaultScale)
    , curScale(DefaultScale)
{
    help = tr("Zoom with mouse wheel, +/- keys or pinch.  Scroll with arrow keys or by dragging.");
    connect(&thread, &render_thread::RenderThread::renderedImage, this, &Mandelbrot::updatePixmap);

    setWindowTitle(tr("Mandelbrot"));
#if QT_CONFIG(cursor)
    setCursor(Qt::CrossCursor);
#endif
}

void Mandelbrot::paintEvent(QPaintEvent * /* event */)
{
    QPainter painter(this);
    painter.fillRect(rect(), Qt::black);

    if (pixmap.isNull()) {
        painter.setPen(Qt::white);
        painter.drawText(rect(),
                         Qt::AlignCenter | Qt::TextWordWrap,
                         tr("Rendering initial image, please wait..."));

        return;
    }

    if (qFuzzyCompare(curScale, pixmapScale)) {
        painter.drawPixmap(pixmapOffset, pixmap);
    } else {
        const auto previewPixmap = qFuzzyCompare(pixmap.devicePixelRatio(), qreal(1))
                                       ? pixmap
                                       : pixmap.scaled(pixmap.deviceIndependentSize().toSize(),
                                                       Qt::KeepAspectRatio,
                                                       Qt::SmoothTransformation);
        const double scaleFactor = pixmapScale / curScale;
        const int newWidth = int(previewPixmap.width() * scaleFactor);
        const int newHeight = int(previewPixmap.height() * scaleFactor);
        const int newX = pixmapOffset.x() + (previewPixmap.width() - newWidth) / 2;
        const int newY = pixmapOffset.y() + (previewPixmap.height() - newHeight) / 2;

        painter.save();
        painter.translate(newX, newY);
        painter.scale(scaleFactor, scaleFactor);

        QRectF exposed = painter.transform().inverted().mapRect(rect());
        exposed = exposed.adjusted(-1, -1, 1, 1);
        painter.drawPixmap(exposed, previewPixmap, exposed);
        painter.restore();
    }

    const QFontMetrics metrics = painter.fontMetrics();
    if (!info.isEmpty()) {
        const int infoWidth = metrics.horizontalAdvance(info);
        const int infoHeight = (infoWidth / width() + 1) * (metrics.height() + 5);

        painter.setPen(Qt::NoPen);
        painter.setBrush(QColor(0, 0, 0, 127));
        painter.drawRect((width() - infoWidth) / 2 - 5, 0, infoWidth + 10, infoHeight);

        painter.setPen(Qt::white);
        painter.drawText(rect(), Qt::AlignHCenter | Qt::AlignTop | Qt::TextWordWrap, info);
    }

    const int helpWidth = metrics.horizontalAdvance(help);
    const int helpHeight = (helpWidth / width() + 1) * (metrics.height() + 5);

    painter.setPen(Qt::NoPen);
    painter.setBrush(QColor(0, 0, 0, 127));
    painter.drawRect((width() - helpWidth) / 2 - 5,
                     height() - helpHeight,
                     helpWidth + 10,
                     helpHeight);

    painter.setPen(Qt::white);
    painter.drawText(rect(), Qt::AlignHCenter | Qt::AlignBottom | Qt::TextWordWrap, help);
}

void Mandelbrot::resizeEvent(QResizeEvent * /* event */)
{
    thread.render(centerX, centerY, curScale, size().width(), size().height(), devicePixelRatio());
}

void Mandelbrot::keyPressEvent(QKeyEvent *event)
{
    switch (event->key()) {
    case Qt::Key_Plus:
        zoom(ZoomInFactor);
        break;
    case Qt::Key_Minus:
        zoom(ZoomOutFactor);
        break;
    case Qt::Key_Left:
        scroll(-ScrollStep, 0);
        break;
    case Qt::Key_Right:
        scroll(+ScrollStep, 0);
        break;
    case Qt::Key_Down:
        scroll(0, -ScrollStep);
        break;
    case Qt::Key_Up:
        scroll(0, +ScrollStep);
        break;
    case Qt::Key_Q:
        close();
        break;
    default:
        QWidget::keyPressEvent(event);
    }
}

#if QT_CONFIG(wheelevent)
void Mandelbrot::wheelEvent(QWheelEvent *event)
{
    const int numDegrees = event->angleDelta().y() / 8;
    const double numSteps = numDegrees / double(15);
    zoom(pow(ZoomInFactor, numSteps));
}
#endif

void Mandelbrot::mousePressEvent(QMouseEvent *event)
{
    if (event->button() == Qt::LeftButton)
        lastDragPos = event->position().toPoint();
}

void Mandelbrot::mouseMoveEvent(QMouseEvent *event)
{
    if (event->buttons() & Qt::LeftButton) {
        pixmapOffset += event->position().toPoint() - lastDragPos;
        lastDragPos = event->position().toPoint();
        update();
    }
}

void Mandelbrot::mouseReleaseEvent(QMouseEvent *event)
{
    if (event->button() == Qt::LeftButton) {
        pixmapOffset += event->position().toPoint() - lastDragPos;
        lastDragPos = QPoint();

        const auto pixmapSize = pixmap.deviceIndependentSize().toSize();
        const int deltaX = (width() - pixmapSize.width()) / 2 - pixmapOffset.x();
        const int deltaY = (height() - pixmapSize.height()) / 2 - pixmapOffset.y();
        scroll(deltaX, deltaY);
    }
}

void Mandelbrot::updatePixmap(const QImage &image, double scaleFactor)
{
    if (!lastDragPos.isNull())
        return;

    info = image.text("info");

    pixmap = QPixmap::fromImage(image);
    pixmapOffset = QPoint();
    lastDragPos = QPoint();
    pixmapScale = scaleFactor;
    update();
}

void Mandelbrot::zoom(double zoomFactor)
{
    curScale *= zoomFactor;
    update();
    thread.render(centerX, centerY, curScale, size().width(), size().height(), devicePixelRatio());
}

void Mandelbrot::scroll(int deltaX, int deltaY)
{
    centerX += deltaX * curScale;
    centerY += deltaY * curScale;
    update();
    thread.render(centerX, centerY, curScale, size().width(), size().height(), devicePixelRatio());
}

#ifndef QT_NO_GESTURES
bool Mandelbrot::gestureEvent(QGestureEvent *event)
{
    if (auto *pinch = static_cast<QPinchGesture *>(event->gesture(Qt::PinchGesture))) {
        if (pinch->changeFlags().testFlag(QPinchGesture::ScaleFactorChanged))
            zoom(1.0 / pinch->scaleFactor());
        return true;
    }
    return false;
}

bool Mandelbrot::event(QEvent *event)
{
    if (event->type() == QEvent::Gesture)
        return gestureEvent(static_cast<QGestureEvent *>(event));
    return QWidget::event(event);
}
#endif

Mandelbrot::~Mandelbrot() {}
