#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main(int argc, char *argv[])
{
    size_t n, mark1, mark2, mark3;
    string last_name, first_name;
    vector<pair<string, float>> persons;
    float top1 = 0.0, top2 = 0.0, top3 = 0.0, mark;
    vector<pair<string, float>>::iterator it;

    cin >> n;
    for (int i = 0; i < n; ++i)
    {
        cin >> last_name >> first_name >> mark1 >> mark2 >> mark3;
        mark = (mark1 + mark2 + mark3) / 3.0;
        if (mark > top1)
        {
            top3 = top2;
            top2 = top1;
            top1 = mark;
        }
        else if (mark > top2)
        {
            top3 = top2;
            top2 = mark;
        }
        else if (mark > top3)
        {
            top3 = mark;
        }
        persons.push_back(pair<string, float>(last_name + " " + first_name, mark));
    }
    for (it = persons.begin(); it != persons.end(); ++it)
    {
        if (it->second >= top3)
        {
            cout << it->first << endl;
        }
    }
}