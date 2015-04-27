#include <SFML/Graphics/Font.hpp>
#include <SFML/Graphics/Text.hpp>
#include "GraphicsControl.h"




void GraphicsControl::setWindow(sf::RenderWindow *window) {
    GraphicsControl::window = window;
}


void GraphicsControl::createGui() {
    font.loadFromFile(SOURCES"DejaVuSans.ttf");
    text.setFont(font);
    text.setCharacterSize(20);
    text.setPosition(20, 20);
    text.setString("This is an Example 1 2 3 4 5 6 7 8 9 0");
}


void GraphicsControl::select(FIELD *field) {
    selected = field;
    hasSelected = true;
}


FIELD* GraphicsControl::getSelected() {
    return selected;
}


bool GraphicsControl::isSelected() {
    return hasSelected;
}


void GraphicsControl::handleKeyCode(sf::Keyboard::Key pressed) {
    if (sf::Keyboard::Key::Num0 == pressed
        || sf::Keyboard::Key::Numpad0 == pressed)
        selected->setNum(0);
    if (sf::Keyboard::Key::Num1 == pressed
        || sf::Keyboard::Key::Numpad1 == pressed)
        selected->setNum(1);
    if (sf::Keyboard::Key::Num2 == pressed
        || sf::Keyboard::Key::Numpad2 == pressed)
        selected->setNum(2);
    if (sf::Keyboard::Key::Num3 == pressed
        || sf::Keyboard::Key::Numpad3 == pressed)
        selected->setNum(3);
    if (sf::Keyboard::Key::Num4 == pressed
        || sf::Keyboard::Key::Numpad4 == pressed)
        selected->setNum(4);
    if (sf::Keyboard::Key::Num5 == pressed
        || sf::Keyboard::Key::Numpad5 == pressed)
        selected->setNum(5);
    if (sf::Keyboard::Key::Num6 == pressed
        || sf::Keyboard::Key::Numpad6 == pressed)
        selected->setNum(6);
    if (sf::Keyboard::Key::Num7 == pressed
        || sf::Keyboard::Key::Numpad7 == pressed)
        selected->setNum(7);
    if (sf::Keyboard::Key::Num8 == pressed
        || sf::Keyboard::Key::Numpad8 == pressed)
        selected->setNum(8);
    if (sf::Keyboard::Key::Num9 == pressed
        || sf::Keyboard::Key::Numpad9 == pressed)
        selected->setNum(9);
}


void GraphicsControl::drawGui() {
    window->draw(text);
}

