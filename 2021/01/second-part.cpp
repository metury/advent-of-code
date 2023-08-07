#include<fstream>
#include<string>
#include<iostream>
#include<array>

void prepare_file(){
	std::fstream stream;
	std::ofstream file;
	file.open("output.txt");
	stream.open("input.txt");
	std::string line;
	std::array<int, 3> list = {0};
	while(getline(stream, line)){
			int new_ = stoi(line);
			int i = 0;
			list[0] += new_;
			file << list[0] << std::endl;
			list[1] += new_;
			list[2] += new_;
			list[0] = list[1];
			list[1] = list[2];
			list[2] = 0;
	}
}

int count(){
	std::fstream stream;
	stream.open("output.txt");
	std::string line;
	int last = -1;
	int increase = -2;
	while(getline(stream, line)){
		int new_ = std::stoi(line);
		if (last != -1 && last < new_){
			increase += 1;
		}
		last = new_;
	}
	return increase;
}

int main(int argc, char ** argv){
	prepare_file();
	int result = count();
	std::cout << result << std::endl;
	return 0;
}
