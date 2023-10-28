# Taskmasterd should have at least the same output and same order
strace supervisord 2>&1 | grep '\.conf"' | awk '{print $2}' | cut -d '"' -f2
