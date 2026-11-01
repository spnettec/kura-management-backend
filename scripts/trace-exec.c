/* Linux validation fixture only. Logs exec attempts without arguments or env. */
#define _GNU_SOURCE
#include <errno.h>
#include <linux/audit.h>
#include <linux/ptrace.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

/* Avoid the libc enum/macro overlap with linux/ptrace.h. */
extern long ptrace(int request, ...);

static void resume(pid_t pid, int sig) {
    if (ptrace(PTRACE_SYSCALL, pid, NULL, (void *)(long)sig) == -1 && errno != ESRCH) {
        perror("ptrace resume");
        exit(125);
    }
}

int main(int argc, char **argv) {
    if (argc < 3) return 125;
    FILE *log = fopen(argv[1], "a");
    if (!log) return 125;
    setvbuf(log, NULL, _IOLBF, 0);
    pid_t root = fork();
    if (root == -1) return 125;
    if (!root) {
        fclose(log);
        if (ptrace(PTRACE_TRACEME, 0, NULL, NULL) == -1) _exit(125);
        raise(SIGSTOP);
        execvp(argv[2], argv + 2);
        _exit(126);
    }
    int status;
    if (waitpid(root, &status, 0) != root || !WIFSTOPPED(status)) return 125;
    long options = PTRACE_O_TRACESYSGOOD | PTRACE_O_TRACEFORK | PTRACE_O_TRACEVFORK |
                   PTRACE_O_TRACECLONE | PTRACE_O_TRACEEXEC | PTRACE_O_EXITKILL;
    if (ptrace(PTRACE_SETOPTIONS, root, NULL, (void *)options) == -1) return 125;
    fprintf(log, "trace-start root=%d\n", root);
    resume(root, 0);
    int code = 0;
    for (;;) {
        pid_t pid = waitpid(-1, &status, __WALL);
        if (pid == -1) {
            if (errno == EINTR) continue;
            if (errno == ECHILD) break;
            return 125;
        }
        if (WIFEXITED(status) || WIFSIGNALED(status)) {
            if (pid == root) code = WIFEXITED(status) ? WEXITSTATUS(status) : 128 + WTERMSIG(status);
            continue;
        }
        int sig = WSTOPSIG(status);
        if (sig == (SIGTRAP | 0x80)) {
            struct ptrace_syscall_info info = {0};
            long size = ptrace(PTRACE_GET_SYSCALL_INFO, pid, sizeof(info), &info);
            if (size < 0) {
                perror("ptrace syscall info");
                return 125;
            }
            if (info.op == PTRACE_SYSCALL_INFO_ENTRY &&
                (info.entry.nr == __NR_execve || info.entry.nr == __NR_execveat)) {
                fprintf(log, "execve(pid=%d, syscall=%llu)\n", pid, (unsigned long long)info.entry.nr);
            }
            resume(pid, 0);
        } else {
            resume(pid, sig == SIGTRAP || sig == SIGSTOP ? 0 : sig);
        }
    }
    fprintf(log, "trace-end code=%d\n", code);
    fclose(log);
    return code;
}
