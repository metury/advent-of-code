#include <fstream>
#include <iostream>
#include <vector>
#include <map>
#include <sstream>
#include <algorithm>

const char JOKER = 'J';

std::map<char, int> mapping = {{'A', 12},
	                           {'K', 11},
	                           {'Q', 10},
	                           {'J',  9},
	                           {'T',  8},
	                           {'9',  7},
	                           {'8',  6},
	                           {'7',  5},
	                           {'6',  4},
	                           {'5',  3},
	                           {'4',  2},
	                           {'3',  1},
	                           {'2',  0}};

struct Hand{
	std::string hand;
	int bet;
	int rating;
	void compute_rating(){
		std::map<char, int> counters = {{'A', 0}, {'K', 0}, {'Q', 0}, {'J', 0},
		                                {'T', 0}, {'9', 0}, {'8', 0}, {'7', 0},
		                                {'6', 0}, {'5', 0}, {'4', 0}, {'3', 0},
		                                {'2', 0}};
		for(size_t i = 0; i < hand.size(); ++i)
			++counters[hand[i]];
		size_t fives = 0;
		size_t fours = 0;
		size_t threes = 0;
		size_t twos = 0;
		for(auto&& [key, counter] : counters){
			if(counter == 5) ++fives;
			else if(counter == 4) ++fours;
			else if (counter == 3) ++threes;
			else if (counter == 2) ++twos;
		}
		if(fives == 1) rating = 6;
		else if(fours == 1) rating = 5;
		else if(threes == 1 && twos == 1) rating = 4;
		else if (threes == 1) rating = 3;
		else if (twos == 2) rating = 2;
		else if (twos == 1) rating = 1;
		else rating = 0;
	}
	void compute_joker_rating(){
		std::map<char, int> counters = {{'A', 0}, {'K', 0}, {'Q', 0}, {'J', 0},
		                                {'T', 0}, {'9', 0}, {'8', 0}, {'7', 0},
		                                {'6', 0}, {'5', 0}, {'4', 0}, {'3', 0},
		                                {'2', 0}, {'J' , 0}};
		for(size_t i = 0; i < hand.size(); ++i)
			++counters[hand[i]];
		size_t jokers = counters[JOKER];
		char max;
		int m = -1;
		for(auto&& [key, counter] : counters){
			if(key == JOKER) continue;
			else if(m < counter){
				m = counter;
				max = key;
			}
		}
		size_t fives = 0;
		size_t fours = 0;
		size_t threes = 0;
		size_t twos = 0;
		for(auto&& [key, counter] : counters){
			if(key == JOKER) continue;
			if(key == max){
				if(counter + jokers == 5) ++fives;
				else if(counter + jokers == 4) ++fours;
				else if (counter + jokers == 3) ++threes;
				else if (counter + jokers == 2) ++twos;
			}
			else{
				if(counter == 5) ++fives;
				else if(counter == 4) ++fours;
				else if (counter == 3) ++threes;
				else if (counter == 2) ++twos;
			}
		}
		if(fives == 1) rating = 6;
		else if(fours == 1) rating = 5;
		else if(threes == 1 && twos == 1) rating = 4;
		else if (threes == 1) rating = 3;
		else if (twos == 2) rating = 2;
		else if (twos == 1) rating = 1;
		else rating = 0;
	}
	void read(const std::string& line){
		std::stringstream ss;
		for(size_t i = 0; i < line.size(); ++i){
			if(line[i] == ' '){
				bet = std::stoi(line.substr(i+1));
				break;
			}
			else ss << line[i];
		}
		hand = ss.str();
	}
};

bool operator<(const Hand& h1, const Hand& h2){
	if(h1.rating < h2.rating)
		return true;
	else if(h2.rating < h1.rating)
		return false;
	else{
		size_t i = 0;
		for(; mapping[h1.hand[i]] == mapping[h2.hand[i]] && i < h1.hand.size(); ++i){}
		if (mapping[h1.hand[i]] > mapping[h2.hand[i]])
			return false;
		return true;
	}
}

std::ostream& operator<<(std::ostream& os, const Hand& h){
	os << "Hand: " << h.hand << " rating: " << h.rating << " bet: " << h.bet;
	return os;
}

void read_file(const std::string& file, std::vector<Hand>& hands){
	std::ifstream ifs;
	ifs.open(file);
	std::string line;
	while(getline(ifs, line)){
		Hand h;
		h.read(line);
		hands.push_back(h);
	}
}

int sum(const std::vector<Hand>& hands){
	int my_sum = 0;
	for(size_t i = 0; i < hands.size(); ++i){
		my_sum += (i+1) * hands[i].bet;
	}
	return my_sum;
}

int main(int argc, char** argv){
	std::vector<Hand> hands;
	read_file("INPUT", hands);
	for(auto&& h : hands)
		h.compute_rating();
	std::sort(hands.begin(), hands.end());
	std::cout << "First part: " << sum(hands) << std::endl;
	for(auto&& h : hands)
		h.compute_joker_rating();
	mapping[JOKER] = -1;
	std::sort(hands.begin(), hands.end());
	std::cout << "Second part: " << sum(hands) << std::endl;
	
	return 0;
}
