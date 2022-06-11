#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include <sys/ioctl.h>

#include <linux/input.h>

int main(int argc, char *argv[]) {
	if (argc != 2) {
		fprintf(stderr, "Usage: %s <event device>\n", argv[0]);
		return 1;
	}

	int fd = open(argv[1], O_RDONLY);
	if (fd < 0) {
		perror("open");
		return 2;
	}

	char buf[255] = { 0 };
	char *name = NULL;
	char *phys = NULL;

	int r = ioctl(fd, EVIOCGNAME(sizeof(buf)), buf);
	if (r < 0) {
		perror("ioctl(EVIOCGNAME)");
	} else {
		name = strdup(buf);
	}

	memset(buf, 0, sizeof(buf));

	r = ioctl(fd, EVIOCGPHYS(sizeof(buf)), buf);
	if (r < 0) {
		perror("ioctl(EVIOCGPHYS)");
	} else {
		phys = strdup(buf);
	}

	printf("%s\n%s\n", name, phys);

	close(fd);
	return 0;
}
