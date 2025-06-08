#include <iostream>
#include <fstream>
#include <cstdlib>

int main() {
    // Clear screen and show title
    system("cls");
    system("title Wolfenstein by AI - Hello World");
    
    // Console output with colors
    system("color 0A");  // Green text on black background
    
    std::cout << "========================================" << std::endl;
    std::cout << "    WOLFENSTEIN BY AI - HELLO WORLD    " << std::endl;
    std::cout << "========================================" << std::endl;
    std::cout << std::endl;
    
    std::cout << "Hello World from Wolfenstein by AI!" << std::endl;
    std::cout << "C++ Game Development Starting..." << std::endl;
    std::cout << "This is our first working C++ program!" << std::endl;
    std::cout << std::endl;
    
    // File output to verify the program runs
    std::ofstream outFile("program_output.txt");
    if (outFile.is_open()) {
        outFile << "Hello World from Wolfenstein by AI!" << std::endl;
        outFile << "C++ Game Development Starting..." << std::endl;
        outFile << "Program executed successfully!" << std::endl;
        outFile.close();
        std::cout << "✓ Output also written to program_output.txt" << std::endl;
    } else {
        std::cout << "✗ Could not create output file" << std::endl;
    }
    
    std::cout << std::endl;
    std::cout << "Program is working correctly!" << std::endl;
    std::cout << "========================================" << std::endl;
    
    // Keep the console window open
    std::cout << std::endl << "Press Enter to exit..." << std::endl;
    std::cin.get();
    
    return 0;
} 