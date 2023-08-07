#include<iostream>
#include<fstream>
#include<string>

int main(int argc, char * * argv){
	std::ifstream stream;
	stream.open("INPUT");
	std::string line;
	int depth = 0;
	int forward = 0;
	int aim = 0;
	while(getline(stream, line)){
		int now = std::stoi(line.substr(line.length()-2, line.length()-1));
		if (line[0] == 'u'){
			aim -= now;
		}
		else if (line[0] == 'f'){
			forward += now;
			depth += (aim*now);
		}
		else{
			aim += now;
		}
	}
	std::cout << "Depth: " << depth << ", forward: " << forward << std::endl;
	long mult = depth*forward;
	std::cout << mult << std::endl; 
}
