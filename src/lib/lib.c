#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>
#include <time.h>
#include <pwd.h>
#include <sys/sysctl.h>
#include <sys/utsname.h>

/* Elements in an array. */
#undef NITEMS
#define NITEMS(x)    (sizeof(x) / sizeof(x[0]))

/* User information structure for __get_userinfo() function. */
struct user_info {
	char *shell;
	char *user;
	char *host;
};

/* Memory information. */
struct mem_info {
	uint64_t total;
	uint64_t free;
};

/* Get the type of BSD system. (e.g. FreeBSD, OpenBSD, NetBSD, etc.). */
char *__get_bsd_type(void)
{
	int mib[2];
	size_t len;
	char *p;

	mib[0] = CTL_KERN;
	mib[1] = KERN_OSTYPE;
	/* Get the length of the sysctl value. */
        if (sysctl(mib, NITEMS(mib), NULL, &len, NULL, 0) == -1)
		return (NULL);

	p = malloc(len + (size_t)1);
	if (p == NULL)
		return (NULL);

	/* Run the sysctl again, to grab the value. */
        if (sysctl(mib, NITEMS(mib), p, &len, NULL, 0) == -1)
		return (NULL);
	return (p);
}

/* Get the release (semver or string literal). */
char *__get_bsd_release(void)
{
	struct utsname uts;
	size_t len;
	char *p;

	if (uname(&uts) == -1)
		return (NULL);

	len = strlen(uts.release) + 1;
	p = malloc(len);
	if (p == NULL)
		return (NULL);

	memcpy(p, uts.release, len);
        return (p);
}

/* Get the system uptime. */
long __get_sys_uptime(void)
{
	size_t len;
	struct timeval tv = {0};
	time_t ti;

	len = sizeof(struct timeval);
	if (sysctlbyname("kern.boottime", &tv,
			 &len, NULL, 0) == -1)
		return (-1);

	ti = time(NULL);
	if (ti == (time_t)-1)
		return (-1);

	return (ti - tv.tv_sec + 30);
}

/* Gather some basic information about the current user. */
int __get_userinfo(struct user_info *ui)
{
	struct passwd *pwd;
	struct utsname uts;
	size_t len;

        pwd = getpwuid(geteuid());
	if (pwd == NULL)
		return (-1);

	if (uname(&uts) == -1)
		return (-1);

	ui->shell = pwd->pw_shell;
	ui->user = pwd->pw_name;
	len = strlen(uts.nodename) + 1;
	ui->host = calloc(len, sizeof(char));
	if (ui->host == NULL)
		return (-1);

	memcpy(ui->host, uts.nodename, len);
	return (0);
}

/* Get the CPU model name. (For FreeBSD, OpenBSD). */
#if defined (__FreeBSD__) || defined (__OpenBSD__)
char *__get_cpu_model(void)
{
	int mib[2];
	size_t len;
	char *p;
	
	mib[0] = CTL_HW;
	mib[1] = HW_MODEL;

	/* Directly use sysctl call as HW_* is predefined for it
	   and avoid a small overhead of using sysctlbyname() function. */
	if (sysctl(mib, NITEMS(mib), NULL, &len, NULL, 0) == -1)
		return (NULL);

	p = malloc(len + 1);
	if (p == NULL)
	        return (NULL);

	if (sysctl(mib, NITEMS(mib), p, &len, NULL, 0) == -1)
		return (NULL);

	return (p);
}

/* Get the CPU model name. (For NetBSD). */
#elif defined (__NetBSD__)
char *__get_cpu_model(void)
{
	size_t len;
	char *p;

	/* Yeah, I *couldn't* be able to find any predefinations
	   of machdep.*. */ 
        if (sysctlbyname("machdep.cpu_brand", NULL,
			 &len, NULL, 0) == -1)
		return (NULL);

	p = malloc(len + 1);
	if (p == NULL)
	        return (NULL);

        if (sysctlbyname("machdep.cpu_brand", p,
			 &len, NULL, 0) == -1)
		return (NULL);
	return (p);
}
/* Ignore others. */
#else
# error Unknown system.
#endif

/* Get the number of CPUs. */
int __get_num_cpus(void)
{
	int mib[2];
	size_t len;
	int ncpus;
	
	mib[0] = CTL_HW;
	mib[1] = HW_NCPU;

	len = sizeof(ncpus);
	if (sysctl(mib, 2, &ncpus, &len, NULL, 0) == -1)
		return (-1);
        return (ncpus);
}

/* Get basic memory information (For FreeBSD). */
#if defined (__FreeBSD__)
int __get_mem_info(struct mem_info *mi)
{
	uint64_t total_page, free_page;
	size_t len;

	total_page = free_page = 0;
	len = sizeof(total_page);
	if (sysctlbyname("vm.stats.vm.v_page_count", &total_page,
			 &len, NULL, 0) == -1)
		return (-1);

	len = sizeof(free_page);
	if (sysctlbyname("vm.stats.vm.v_free_count", &free_page,
			 &len, NULL, 0) == -1)
		return (-1);

	mi->total = total_page;
	mi->free = free_page;
	return (0);
}

/* No exposed sysctl key? That's pretty weird, why does
   every BSDs has to compete with different sysctl keys? */
#elif defined (__OpenBSD__) || defined (__NetBSD__)
int __get_mem_info(struct mem_info *mi)
{
	uint64_t total_page;
	size_t len;

	total_page = 0;
	len = sizeof(total_page);
	if (sysctlbyname("hw.usermem64", &total_page,
			 &len, NULL, 0) == -1)
		return (-1);

	mi->total = total_page;
	mi->free = 0;
	return (0);
}
/* Ignore others. */
#else
# error Unsupport system.
#endif

/* Wrapper function for getpagesize(). */
int __getpagesize(void)
{
	return (getpagesize());
}
