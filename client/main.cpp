#include "client.h"

int main(int argc, char** argv) {
    Client client;
    client.CLIParse(argc, argv);
    client.runCLI();
    
    return 0;
}
