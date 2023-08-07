#include<iostream>
#include<fstream>
#include<string>
#include<array>
#include<vector>

struct Square{
    public:
        int number;
        bool used;
};

using board = std::array< std::array < Square, 5>, 5>;

void read(std::vector<int>& numbers, std::vector<board>& boards){
    std::ifstream file;
    file.open("INPUT");
    std::string line;
    bool first = true;
    while (getline(file, line)){
        if (first){
            for (size_t i = 0; i < line.length(); ++i){
                size_t shift = 0;
                int now = std::stoi(line.substr(i, line.length()), &shift);
                i += shift;
                numbers.push_back(now);
            }
            first = false;
            getline(file, line);
        }
        else{
            board temp;
            for (size_t j = 0; j < 5; ++j){
                size_t l = 0;
                for (size_t i = 0; i < line.length(); ++i){
                    Square sq;
                    size_t shift;
            		int now = std::stoi(line.substr(i, line.length()), &shift);
                    i += shift;
            		sq.number = now;
                    temp[j][l] = sq;
                    ++l;
                }
                getline(file, line);
            }
            boards.push_back(temp);
        }
    }
}

bool check (board& brd){
    std::array<bool, 5> rows = {true, true, true, true, true};
    std::array<bool, 5> columns = {true, true, true, true, true};
    bool succes = false;
    for (size_t i = 0; i < 5; ++i){
        for (size_t j = 0; j < 5; ++j){
            rows[i] = rows[i] && brd[i][j].used;
            columns[j] = columns[j] && brd[i][j].used;
        }
    }
    for (size_t i = 0; i < 5; ++i){
        succes = succes || rows[i];
        succes = succes || columns[i];
    }
    return succes;
}

int calculate(int number, board& brd){
    int result = 0;
    for (size_t i = 0; i < 5; ++i){
        for (size_t j = 0; j < 5; ++j){
            if (!brd[i][j].used){
                result += brd[i][j].number;
            }
        }
    }
    std::cout << "Sum: " << result << std::endl;
    result *= number;
    return result;
}

void print_matrix(board& brd){
std::cout << "Matrix:" << std::endl;
for (size_t i = 0; i < 5; ++i){
    for (size_t j = 0; j < 5; ++j){
            std::cout << brd[i][j].number << ":" << brd[i][j].used << " ";
    }
std::cout << std::endl;
}
}

int compute(std::vector<int>& numbers, std::vector<board>& boards){
    for (auto& num : numbers){
        for (auto& brd : boards){
            for (size_t i = 0; i < 5; ++i){
                for (size_t j = 0; j < 5; ++j){
                    if(brd[i][j].number == num){
                        brd[i][j].used = true;					
                    }
                }
            }
            if (check(brd)){
                std::cout << "Number: " << num << std::endl;
                print_matrix(brd);
                return calculate(num, brd);
            }
        }
    }
    return 0;
}

int main(int argc, char * * argv){
    std::vector<int> numbers;
    std::vector<board> boards;
    read(numbers, boards);
    int result = compute(numbers, boards);
    std::cout << result << std::endl;
    return 0;
}
