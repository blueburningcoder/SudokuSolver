#include "Sudoku.h"




// initiatiing all necessary information for later reference
void FIELD::init(int x, int y, sf::Font newFont, int Cluster,
                 Sudoku *sud, int index) {
    font = newFont;
    xPos = x;
    yPos = y;
    ClusterNum = Cluster;
    sudoku = sud;
    Index = index;
    Tile.setPosition(xPos, yPos);
    Tile.setFillColor(sf::Color::Black);
    Tile.setSize(sf::Vector2f(48, 48) );

    for (int i = 1; i < 11; i++) {
        Nums[i - 1].setFont(font);
        Nums[i - 1].setPosition(x + 2, y);
        Nums[i - 1].setCharacterSize(14);
        Nums[i - 1].setColor(sf::Color(20, 200, 20) );
        Nums[i - 1].setString(std::to_string(i) );
        x += 16;
        if (i % 3 == 0) {
            y += 15;
            x = xPos;
        }
    }
    Nums[9].setCharacterSize(40);
    Nums[9].setColor(sf::Color(60, 60, 255) );
    Nums[9].setPosition(xPos + 10, yPos);
    Nums[9].setString(std::to_string(index) );

    for (int i = 0; i < 9; i++) {
        possible.push_back(true);
    }

    isInit = true;
}


// drawing the field
void FIELD::draw(sf::RenderWindow *window) {
    if (isInit) {
        window->draw(Tile);

        if (num == NOTHING) { // change this to show the Index of the field
            for (int i = 0; i < 9; i++)
                window->draw(Nums[i]);
        } else
            window->draw(Nums[9]);
    }
}


// updates the field, TODO: multithreading
void FIELD::update() {
    if (num == NOTHING) {
        for (int i = 1; i < 10; i++) {
            setPossible((NUMBERS) i);
        }
        for (int dir = 0; dir < 4; dir++) {
            FIELD *next = this;
            bool ongoing = true;
            while (ongoing) {
                NUMBERS othNum = next->getNum();
                if (othNum != NOTHING) {
                    removePossible(othNum);
                }
                ongoing = next->isNeighbour(dir);
                next = next->getNeighbour(dir);
            }
        }
        int lowestIndInCluster = sudoku->getLowestIndexInCluster(ClusterNum);
        for (int i = 0; i < 9; i++) {
            FIELD* next = sudoku->getNextFromCluster(lowestIndInCluster, ClusterNum);
            if (next != NULL) {
                NUMBERS othNum = next->getNum();
                if (othNum != NOTHING)
                    removePossible(othNum);
                lowestIndInCluster = next->Index;
            }
        }
    }
}


// setting the number to @param number
void FIELD::setNum(int number) {
    num = (NUMBERS) number;

    if (number % 10 == number && num != NOTHING)
        Nums[9].setString(std::to_string(num));
}


// removing the possibility of the @param alredyNum
void FIELD::removePossible(NUMBERS alreadyNum) {
    possible[(int) alreadyNum] = false;
    Nums[(int) alreadyNum - 1].setColor(sf::Color(200, 20, 20, 150) );
}


// setting @param possNum possible (if it wasn't before)
void FIELD::setPossible(NUMBERS possNum) {
    possible[(int) possNum] = true;
    Nums[(int) possNum - 1].setColor(sf::Color(20, 200, 20) );
}


// @return the number the field currently has
NUMBERS FIELD::getNum() {
    return num;
}


// setting the Neighbour in @param dir to @param next
void FIELD::setNeighbour(int dir, FIELD *next) {
    if (dir % 4 == dir)
        Neighbours[dir] = next;
}


// returns if there is a Neighbour in @param dir
bool FIELD::isNeighbour(int dir) {
    return dir % 4 == dir && Neighbours[dir] != NULL;
}


// @return the Neighbour in @param dira
FIELD* FIELD::getNeighbour(int dir) {

    if (dir % 4 == dir && isNeighbour(dir) ) {
        return Neighbours[dir];
    }

    return NULL;
}


// @return if the _mouse_ is inside the field
bool FIELD::isInside(int x, int y) {
    return xPos <= x && xPos + 48 >= x && yPos <= y && yPos + 48 >= y;
}


// @return the ClusterID
int FIELD::getClusterNum() {
    return ClusterNum;
}





// initiating the Sudoku, can be drawn from that point forward
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

    std::cout << "initiating fields" << std::endl;

    for (int num = 0; num < 9; num ++) {
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                fields[i + (num / 3) * 3][j + (num % 3) * 3].init(i * 50 + moveX,
                        j * 50 + moveY, font, num, this ,
                        j + i * 3 + num * 9);
            }
        }
        moveY += 152;
        if (moveY > 500) {
            moveY = 50;
            moveX += 152;
        }
    }

    std::cout << "setting neighbours" << std::endl;

    setNeighbours();

    std::cout << "Finished Sudoku init" << std::endl;

}


// sets the Neighbours of the fields
void Sudoku::setNeighbours() {
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            if (j > 0)
                fields[i][j].setNeighbour(0, &fields[i][j - 1]);
            if (i < 9 - 1)
                fields[i][j].setNeighbour(1, &fields[i + 1][j]);
            if (j < 9 - 1)
                fields[i][j].setNeighbour(2, &fields[i][j + 1]);
            if (i > 0)
                fields[i][j].setNeighbour(3, &fields[i - 1][j]);
        }
    }
}


// draws the Sudoku on the @param window
void Sudoku::draw(sf::RenderWindow *window) {

    Update();

    window->draw(Ground);

    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            fields[i][j].draw(window);
        }
    }
}


// setting at field @param i and j the @param number
void Sudoku::setNum(int i, int j, int number) {
    if (i % 9 == i && j % 9 == j && number % 10 == number)
        fields[i][j].setNum(number);
}

// @return the field that got clicked or NULL
FIELD* Sudoku::getClicked(int x, int y) {
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            if (fields[i][j].isInside(x, y) )
                return &fields[i][j];
        }
    }
    return NULL;
}


// lets update all the fields for themselves
void Sudoku::Update() {
    // TODO: Multithreading potential


    for (int i = 0; i < 9; i++)
        for (int j = 0; j < 9; j++)
            fields[i][j].update();
}


// @return the next field starting fom @param index from the @param Cluster
FIELD* Sudoku::getNextFromCluster(int index, int Cluster) {
    int currentInd = index;



    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            if (fields[i][j].getClusterNum() == Cluster && fields[i][j].Index > index) {
                return &fields[i][j];
            }
        }
    }

    return NULL;
}


// @return the lowest Num from the @param Cluster
int Sudoku::getLowestIndexInCluster(int Cluster) {
    int lowestNum = 0;
    for (int i = 0; i < 9; i ++) {
        for (int j = 0; j < 9; j ++) {
            if (fields[i][j].getClusterNum() == Cluster && fields[i][j].Index < lowestNum)
                lowestNum = fields[i][j].Index;
        }
    }

    return lowestNum;
}



