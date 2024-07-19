#include <vector>

#define N_SPIRES 4
#define N_BALLS 4

using namespace std;


class State {
    private:
        int movesSoFar;
        vector<vector<int>> val;

        vector<State> possibleNextMoves;

        void computePossibleNextMoves(void);

        State moved(int fromSpire, int toSpire);
    
    public:
        static State fromString(char *str);

        State(vector<vector<int>> val, int movesSoFar = 0);

        vector<State> getPossibleNextMoves(void);

        int getMovesSoFar(void);

        vector<int> operator[](int i);
        bool operator==(State other);
};

int findMinMoves(State startState, State endState);
