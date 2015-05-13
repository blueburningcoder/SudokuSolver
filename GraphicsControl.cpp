#include <SFML/Graphics/Font.hpp>
#include <SFML/Graphics/Text.hpp>
#include "GraphicsControl.h"



// init
debugText::debugText() {
    font.loadFromFile(SOURCES"DejaVuSans.ttf");
    for (int i = 0; i < 4; i++) {
        debugCon[i].setFont(font);
        debugCon[i].setCharacterSize(8);
        debugCon[i].setPosition(10, 510 + i * 10);
        debugCon[i].setString("Console init, please wait .... ");
    }
}


void debugText::add(std::string text) {
    debugCon[3].setString(debugCon[2].getString() );
    debugCon[2].setString(debugCon[1].getString() );
    debugCon[1].setString(debugCon[0].getString() );
    debugCon[0].setString(text);

}


// drawing the debugTexts
void debugText::draw(sf::RenderWindow *window) {
    for (int i = 0; i < 4; i++) {
        window->draw(debugCon[i]);
    }
}

// setting the @param window for drawing on it later on
void GraphicsControl::setWindow(sf::RenderWindow *window) {
    GraphicsControl::window = window;
    out("Window set ...");
}


// creates the GUI ... TODO: update
void GraphicsControl::createGui() {
    font.loadFromFile(SOURCES"DejaVuSans.ttf");
    text.setFont(font);
    text.setCharacterSize(20);
    text.setPosition(200, 6);
    text.setString("Solve  -  Create");
    out("GUI created ...");
}


// selects the @param field if it isn't null
void GraphicsControl::select(FIELD *field) {
    hasSelected = field != NULL;

    if (hasSelected) {
        selected = field;
        out("Selected Field: " + std::to_string(field->Index)
            + " Cluster: " + std::to_string(field->getClusterNum() ) );
    }

}


// returns the latest selected field or null
FIELD* GraphicsControl::getSelected() {
    return selected;
}


// returns if there is an field selected currrently
bool GraphicsControl::isSelected() {
    return hasSelected;
}


// handles the @param pressed key (only numkeys though)
void GraphicsControl::handleKeyCode(sf::Keyboard::Key pressed) {
    if (selected != NULL) {
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
    } else out("NO SELECTED FIELD");
}


// drawing the gui
void GraphicsControl::drawGui() {
    window->draw(text);
    dbg.draw(window);
}


// adding the @param text to the output
void GraphicsControl::out(std::string text) {
    dbg.add(text);
    std::cout << "from Console: " << text << std::endl;
}


