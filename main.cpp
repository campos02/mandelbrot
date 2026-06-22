#include "mandelbrot.h"

#include <QApplication>
#include <mandelbrot_rust/src/render_thread.cxxqt.h>

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);
    Mandelbrot w;
    w.show();
    return a.exec();
}
