#include <queue>
#include <set>
#include <algorithm>

#include "q3.h"

const std::string ALPHABET = "ABCDEFGHIJKLNOPQRSTUVWXYZ";


class State {
    private:
        std::string display;
        std::string next_boxes;
        int n_moves;

    public:
        State(std::string display, std::string next_boxes) {
            this->display = display;
            this->next_boxes = next_boxes;
            n_moves = 0;
        }

        void add() {
            if (next_boxes.length() > 0) {
                display += next_boxes.front();
                next_boxes.erase(0, 1);
                ++n_moves;
            }
        }

        void swap() {
            if (display.length() >= 2) {
                char temp = display[0];
                display[0] = display[1];
                display[1] = temp;
                ++n_moves;
            }
        }

        void rotate() {
            if (display.length() >= 2) {
                display += display[0];
                display.erase(0, 1);
                ++n_moves;
            }
        }

        int get_n_moves() {
            return n_moves;
        }

        bool operator==(const State& other) const {
            return display == other.display && next_boxes == other.next_boxes;
        }

        bool operator<(const State& other) const {
            if (display != other.display) {
                return display < other.display;
            } else {
                return next_boxes < other.next_boxes;
            }
        }
};

void add_if_not_contained(std::queue<State>& next_states, std::set<State>& visited_states, State& new_state) {
    if (std::find(visited_states.begin(), visited_states.end(), new_state)
         == visited_states.end()) {
        next_states.push(new_state);
        visited_states.insert(new_state);
    }
}

int find_min_moves(std::string display_order) {
    State display_order_state = State(display_order, "");

    std::queue<State> next_states;
    
    State selected_state("", ALPHABET.substr(0, display_order.length()));

    std::set<State> visited_states;
    visited_states.insert(selected_state);
    
    while (true) {
        State added_state = selected_state;
        added_state.add();
        if (added_state == display_order_state) {
            return added_state.get_n_moves();
        }
        add_if_not_contained(next_states, visited_states, added_state);

        State swapped_state = selected_state;
        swapped_state.swap();
        if (swapped_state == display_order_state) {
            return swapped_state.get_n_moves();
        }
        add_if_not_contained(next_states, visited_states, swapped_state);

        State rotated_state = selected_state;
        rotated_state.rotate();
        if (rotated_state == display_order_state) {
            return rotated_state.get_n_moves();
        }
        add_if_not_contained(next_states, visited_states, rotated_state);

        selected_state = next_states.front();
        next_states.pop();
    }
}

