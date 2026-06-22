#ifndef MANDELBROT_H
#define MANDELBROT_H

#include <QWidget>

class Mandelbrot : public QWidget
{
    Q_OBJECT

public:
    Mandelbrot(QWidget *parent = nullptr);
    ~Mandelbrot();
};

#endif // MANDELBROT_H
