#include <sys/socket.h>
#include <sys/types.h>
#include <netinet/in.h>
#include <netdb.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>

#include "socket_wrapper.h"

int recv_socket;

int socket_init(int port){
        int true = 1;
        struct sockaddr_in server_addr;
        if ((recv_socket = socket(AF_INET, SOCK_STREAM, 0)) == -1){
            perror("Socket");
            return -1;
        }

	if (setsockopt(recv_socket,SOL_SOCKET,SO_REUSEADDR,&true,sizeof(int)) == -1) {
            perror("Setsockopt");
            return -1;
        }

	server_addr.sin_family = AF_INET;         
        server_addr.sin_port = htons(port);     
        server_addr.sin_addr.s_addr = INADDR_ANY; 
        bzero(&(server_addr.sin_zero),8); 

	if (bind(recv_socket, (struct sockaddr *)&server_addr, sizeof(struct sockaddr))== -1) {
            perror("Unable to bind");
            return -1;
        }

	if (listen(recv_socket, 5) == -1) {
            perror("Listen");
            return -1;
        }
        
        printf("\nTCPServer Waiting for client on port %d\n", port);
}


void socket_close(){
        close(recv_socket);
}

int send_data(char *ip, int port, char *buffer, int size){
	
	int sock, bytes_recieved;
	struct hostent *host;
        struct sockaddr_in server_addr;

	host = gethostbyname(ip);
	
	if ((sock = socket(AF_INET, SOCK_STREAM, 0)) == -1){
            perror("Socket");
            return -1;
        }

	server_addr.sin_family = AF_INET;     
        server_addr.sin_port = htons(port);   
        server_addr.sin_addr = *((struct in_addr *)host->h_addr);
        bzero(&(server_addr.sin_zero),8);

	if (connect(sock, (struct sockaddr *)&server_addr,
                    sizeof(struct sockaddr)) == -1){
            perror("Connect");
            return -1;
        } 

	send(sock,buffer,size, 0);   
        close(sock);
	return 0;
}

int recv_data(int port, char *buffer, int size){

	int connected, bytes_recieved;  
	struct sockaddr_in client_addr;    
        int sin_size;

	sin_size = sizeof(struct sockaddr_in);
        connected = accept(recv_socket, (struct sockaddr *)&client_addr,&sin_size);
	bytes_recieved = recv(connected,buffer,size,0);
	close(connected);

	return bytes_recieved;
}


