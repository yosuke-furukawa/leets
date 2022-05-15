#include <iostream>
#include <thread>
using namespace std;

class Foo {
public:
    mutex a, b;
    Foo() {
        a.lock();
        b.lock();        
    }

    void first(function<void()> printFirst) {
        // printFirst() outputs "first". Do not change or remove this line.
        printFirst();
        a.unlock();
    }

    void second(function<void()> printSecond) {
        
        // printSecond() outputs "second". Do not change or remove this line.
        a.lock();
        printSecond();
        b.unlock();
    }

    void third(function<void()> printThird) {
        
        // printThird() outputs "third". Do not change or remove this line.
        b.lock();
        printThird();
    }
};

void printFirst() {
  cout << "first";
}

void printSecond() {
  cout << "secnd";
}

void printThird() {
  cout << "third";
}