# Mandelbrot
Este é um dos exemplos de uso do Qt, sendo um programa que gera fractais de Mandelbrot em uma thread separada e envia as imagens à interface
gráfica conforme elas ficam prontas. Enquanto o [original](https://doc.qt.io/qt-6/qtcore-threads-mandelbrot-example.html) é completamente C++, 
neste repositório o código da thread geradora é escrito em Rust.

## Arquivos
- main.cpp, mandelbrot.cpp e mandelbrot.h: contém o código da interface gráfica, feita com Qt, que recebe os sinais emitidos pela thread
e exibe os fractais gerados, além de reagir à interação do usuário solicitando novas imagens.
- CMakeLists.txt: contém as instruções para o CMake compilar o projeto.

### Pasta rust
- Cargo.toml: especifica dependências e que o programa deve ser compilado como uma biblioteca estática.
- build.rs: script de compilação especificando quais arquivos devem ser ligados com o código C++
- src/lib.rs: torna o módulo render_thread público
- src/render_thread.rs: contém o código completo que gera os fractais, cria sua thread e emite os sinais para a interface gráfica

## Compilação e execução
> [!IMPORTANT]
> Necessário ter Qt, CMake e Rust instalados, além do qmake na variável PATH

Para compilar rodar os comandos:
``````
cmake -S . -B build
cmake --build build
``````

O executável estará em `build/examples/mandelbrot/mandelbrot`
