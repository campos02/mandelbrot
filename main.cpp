#include "mandelbrot.h"

#include <QApplication>

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);
    Mandelbrot widget;
    widget.grabGesture(Qt::PinchGesture);
    widget.show();
    return app.exec();
}
