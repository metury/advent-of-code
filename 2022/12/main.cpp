#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <climits>
#include <queue>

int start[] = {0,0};
int end[] = {0,0};
std::vector<int> sPoints;

std::vector<std::vector<int>> read(){
	std::ifstream input = std::ifstream();
	input.open("INPUT");
	std::string line;
	std::vector<std::vector<int>> heights;
	int x = 0;
	while(getline(input, line)){
		heights.push_back(std::vector<int>());
		for(size_t i = 0; i < line.length(); ++i){
			char c = line[i];
			if(c == 'S'){
				start[0] = x;
				start[1] = i;
				heights[x].push_back(0);
			}
			else if(c == 'E'){
				end[0] = x;
				end[1] = i;
				heights[x].push_back(int('z') - int('a'));
			}
			else if(c != '\n'){
				heights[x].push_back(int(c) - int('a'));
			}
			if (c == 'a'){
				int y = i;
				sPoints.push_back(x);
				sPoints.push_back(y);
			}
		}
		++x;
	}
	return heights;
}

void onePath(std::queue<int>& q, int x, int y, const std::vector<std::vector<int>>& h, std::vector<std::vector<int>>& v, int p0, int p1, int step){
	bool in_matrix = (p1 + x >= 0) && (p1 + y >= 0) && (p0 + x < h.size()) && (p1 + y < h[0].size());
	if(!in_matrix){
		return;
	}
	bool possible =  h[p0 + x][p1 + y] <= h[p0][p1] + 1;
	if(!possible){
		return;
	}
	bool worth = v[p0 + x][p1 + y] > step + 1;
	if(!worth){
		return;
	}
	v[p0 + x][p1 + y] = step + 1;
	if (end[0] == p0 + x && end[1] == p1 + y){
		return;
	}
	q.push(p0+x);
	q.push(p1+y);
	q.push(++step);
}

int findPath(const std::vector<std::vector<int>>& h, int s0, int s1){
	std::queue<int> q;
	std::vector<std::vector<int>> visited;
	for(size_t i = 0; i < h.size(); ++i){
		visited.push_back(std::vector<int>());
		for(size_t j = 0; j < h[i].size(); ++j){
			visited[i].push_back(INT_MAX);
		}
	}
	visited[s0][s1] = 0;
	q.push(s0);
	q.push(s1);
	q.push(0);
	int neigh[][2] = {{-1, 0}, {1, 0}, {0, 1}, {0,-1}};
	while(!q.empty()){
		int p0 = q.front();
		q.pop();
		int p1 = q.front();
		q.pop();
		int step = q.front();
		q.pop();
		for(auto && n : neigh){
			onePath(q, n[0], n[1], h, visited, p0, p1, step);
		}
	}
	return visited[end[0]][end[1]];
}

int main(int argc, char * * argv){
	std::vector<std::vector<int>> h = read();
	int min = findPath(h, start[0], start[1]);
	std::cout << "Prvni: " << min << std::endl;
	std::cout << "Druhy: ";
	for(size_t i = 0; i < sPoints.size(); i += 2){
		int tmp = findPath(h, sPoints[i], sPoints[i+1]);
		if(tmp < min){
			min = tmp;
		}
	}
	std::cout << min << std::endl;
	return 0;
}
