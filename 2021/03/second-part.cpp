#include<iostream>
#include<fstream>
#include<string>
#include<array>

int write_file(std::string finput, std::string foutput, int compare, char comparison){
    std::ifstream input;
    std::ofstream output;
    input.open(finput);
    output.open(foutput);
    std::string line;
    int number = 0;
    while(getline(input, line)){
        if (line[compare] == comparison){
            output << line << std::endl;
            ++number;
        }
    }
    return number;
}

void compute(std::string f1, std::string f2){
    std::ifstream i1;
    std::ifstream i2;
    i1.open(f1);
    i2.open(f2);
    std::string line;
    size_t p;
    getline(i1, line);
    int first = stoi(line, &p, 2);
    getline(i2, line);
    int second = stoi(line, &p, 2);
    std::cout << "First: " << first << std::endl;
    std::cout << "Second: " << second << std::endl;
    std::cout << first*second << std::endl;
}

char filter_most(std::string file, int i){
    std::fstream stream;
    stream.open(file);
    std::string line;
    const size_t length = 12;
    int ones = 0;
    int zeros = 0;
    while(getline(stream, line)){
        if (line[i] == '0'){
            if (ones >= 1){
                ones -= 1;
            }
            else{
                zeros += 1;
            }
        }
        else if (line[i] == '1'){
            if (zeros >= 1){
                zeros -= 1;
            }
            else{
                ones += 1;
            }
        }
    }
    if (zeros == 0){
        return '1';
    }
    else{
        return '0';
    }
}

char filter_least(std::string file, int i){
    std::fstream stream;
    stream.open(file);
    std::string line;
    const size_t length = 12;
    int ones = {0};
    int zeros = {0};
    while(getline(stream, line)){
        if (line[i] == '0'){
            if (ones >= 1){
                ones -= 1;
            }
            else{
                zeros += 1;
            }
        }
        else if (line[i] == '1'){
            if (zeros >= 1){
                zeros -= 1;
            }
            else{
                ones += 1;
            }
        }
    }
    if (zeros == 0){
        return '0';
    }
    else{
        return '1';
    }
}

int main(int argc, char * * argv){
    std::string first = "1";
    std::string second = "2";
    char gamma = filter_most(first, 0);
    int i = 0;
    while (write_file(first, second, i, gamma) > 1){
        std::string temp = second;
        second = first;
        first = temp;
        ++i;
        gamma = filter_most(first, i);
    }
    first = "3";
    second = "4";
    char epsilon = filter_least(first, 0);
    i = 0;
    while (write_file(first, second, i, epsilon) > 1){
            std::string temp = second;
            second = first;
            first = temp;
            ++i;
            epsilon = filter_least(first, i);
        }
    first = "3";
    second = "1";
    compute(first, second);
    return 0;
}

