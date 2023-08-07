#include<fstream>
#include<string>
#include<iostream>

int main(int argc, char ** argv){
	std::fstream stream;
	stream.open("INPUT");
	std::string line;
	int last = -1;
	int increase = 0;
	while(getline(stream, line)){
		int new_ = std::stoi(line);
		if (last != -1 && last < new_){
			increase += 1;
		}
		last = new_;
	}
	std::cout << increase << std::endl;
	return 0;
}
