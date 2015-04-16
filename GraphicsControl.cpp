#include <SFML/Graphics/Font.hpp>
#include <SFML/Graphics/Text.hpp>
#include "GraphicsControl.h"




void GraphicsControl::setWindow(sf::RenderWindow *window) {
    GraphicsControl::window = window;
}


void GraphicsControl::createGui() {
    font.loadFromFile(SOURCES"DejaVuSans.ttf");
    text.setFont(font);
    text.setCharacterSize(30);
    text.setPosition(20, 20);
    text.setString("This is an Example 1 2 3 4 5 6 7 8 9 0");
}


void GraphicsControl::drawGui() {
    window->draw(text);
}

