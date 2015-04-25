#include "Sudoku.h"




void FIELD::init(int x, int y, sf::Font newFont) {
    font = newFont;
    Tile.setPosition(x, y);
    Tile.setFillColor(sf::Color::Black);
    Tile.setSize(sf::Vector2f(48, 48) );

    text.setFont(font);
    text.setPosition(x + 2, y);
    text.setCharacterSize(14);
    text.setColor(sf::Color(200, 200, 200) );

    std::string test = "";

    for (int i = 1; i < 10; i++) {
        test += std::to_string(i);
        test += "  ";
        if (i % 3 == 0)
            test += "\n";
    }

    text.setString(test);

    isInit = true;
}


void FIELD::draw(sf::RenderWindow *window) {
    window->draw(Tile);

    if (isInit)
        window->draw(text);
}


Sudoku::Sudoku() {
    Ground.setPosition(x - 4, y - 4);
    Ground.setSize(sf::Vector2f(460, 460) );
    Ground.setFillColor(sf::Color::White);

    std::vector<FIELD> tmp1;
    FIELD tmp2;

    std::cout << "setting possible" << std::endl;

    for (int i = 1; i < 10; i++) {
        tmp2.possible.push_back( (NUMBERS) i);
    }

    std::cout << "pushing back tmp1" << std::endl;

    for (int i = 0; i < 9; i++) {
        tmp1.push_back(tmp2);
    }

    std::cout << "pushing back tmp2" << std::endl;

    for (int i = 0; i < 9; i++) {
        fields.push_back(tmp1);
    }

    if (!font.loadFromFile(SOURCES"DejaVuSans.ttf") )
        std::cout << "Failed loading font" << std::endl;

    int moveX = 50, moveY = 50;

    for (int num = 0; num < 9; num ++) {
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                fields[i + (num / 3) * 3][j + (num % 3) * 3].init(i * 50 + moveX,
                        j * 50 + moveY, font);
            }
        }
        moveY += 152;
        if (moveY > 500) {
            moveY = 50;
            moveX += 152;
        }
    }

    /*
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            fields[i][j].init(i * 50 + 50, j * 50 + 50, font);
        }
    }
     */
}


void Sudoku::draw(sf::RenderWindow *window) {

    window->draw(Ground);

    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            fields[i][j].draw(window);
        }
    }
}

