int socket_fd = socket(AF_INET6, SOCK_STREAM, 0);
if (socket_fd < 0) {
	perror("Error creating socket:");
	exit(EXIT_FAILURE);
}
int set = 1;
int ret = setsockopt(socket_fd, SOL_SOCKET,
           SO_REUSEADDR, &set,
           sizeof(set));
if (ret < 0) {
	perror("setsockopt failed");
	return -1;
}
struct sockaddr_in6 server_address;
memset(&server_address, 0, sizeof(server_address));
server_address.sin6_family = AF_INET6;
server_address.sin6_port = htons(port_number);
server_address.sin6_addr = in6addr_any;
ret = bind(socket_fd, 
           (struct sockaddr*)&server_address,
           sizeof(server_address));
if (ret < 0) {
	perror("Bind failed");
	exit(EXIT_FAILURE);
}
ret = listen(socket_fd, 1);
if (ret < 0) {
	perror("Listen failed");
	exit(EXIT_FAILURE);
}
struct sigaction action;
set_signal(&action);
while (running) {
	int new_socket_fd = accept(socket_fd, NULL, NULL);
	if (new_socket_fd < 0) {
		perror("Error accepting");
		break;
	}
	handle_client(new_socket_fd);
	close(new_socket_fd);
}
close(socket_fd);