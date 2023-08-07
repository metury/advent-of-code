#include<iostream>
#include<fstream>
#include<string>
#include<array>

int main(int argc, char * * argv){
    std::fstream stream;
    stream.open("INPUT");
    std::string line;
    const size_t length = 12;
    std::array<int, length> ones = {0};
    std::array<int, length> zeros = {0};
    int gamma = 0;
    int epsilon = 0;
    while(getline(stream, line)){
        for (size_t i = 0; i < length; ++i){
            if (line[i] == '0'){
                if (ones[i] >= 1){
                    ones[i] -= 1;
                }
                else{
                    zeros[i] += 1;
                }
            }
            else if (line[i] == '1'){
                if (zeros[i] >= 1){
                    zeros[i] -= 1;
                }
                else{
                    ones[i] += 1;
                }
            }
        }
    }
    int two = 2048; //16;
    int max = 0;
    for (size_t j = 0; j < length; ++j){
        if (zeros[j] == 0){
            gamma += two;
        }
        else if (ones[j] == 0){
            epsilon += two;
        }
        max += two;
        two /= 2;
    }
    std::cout << "Gamma: " << gamma << std::endl;
    std::cout << "Epsilon: " << epsilon << std::endl;
    std::cout << "Max: " << max << std::endl;
    std::cout << "Sum: " << gamma + epsilon << std::endl;
    long compute = epsilon*gamma;
    std::cout << compute << std::endl;	
    return 0;
}
