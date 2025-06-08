#include <iostream>
#include <fstream>

int main() {
    // Console output
    std::cout << "Hello World from Wolfenstein by AI!" << std::endl;
    std::cout << "C++ Game Development Starting..." << std::endl;
    
    // File output to verify the program runs
    std::ofstream outFile("program_output.txt");
    if (outFile.is_open()) {
        outFile << "Hello World from Wolfenstein by AI!" << std::endl;
        outFile << "C++ Game Development Starting..." << std::endl;
        outFile << "Program executed successfully!" << std::endl;
        outFile.close();
        std::cout << "Output also written to program_output.txt" << std::endl;
    }
    
    // Wait for user input to see the output
    std::cout << "Press Enter to continue..." << std::endl;
    std::cin.get();
    
    return 0;
} 