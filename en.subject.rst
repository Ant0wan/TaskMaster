Taskmaster
===========

Summary
-------
The goal of this project is to create a job control daemon with features similar to supervisor.

Version
-------
3

Contents
--------
I. Foreword
II. Introduction
III. Goals
IV. General Instructions
   IV.1 Language constraints
   IV.2 Defense session
V. Mandatory Part
VI. Bonus Part
VII. Appendix
    VII.1 Example configuration file
    VII.2 Trying out supervisor
VIII. Submission and Peer Correction

Chapter I: Foreword
--------------------
Here's a useful piece of information about the Dwellers:
Picking a fight with the Dwellers, a widespread, long-lived, and irascible species, often led to unexpected consequences. Disputes with them would resurface even after long periods, sometimes spanning geological ages. They would bring a surprise planet into your home system, accompanied by moons and asteroid-sized chunks, all hurtling at nearly the speed of light, leaving little time for warning before disappearing in a blaze of radiation. To understand more about Dwellers, consider reading "The Algebraist."

Chapter II: Introduction
-------------------------
In Unix-like operating systems, job control involves managing jobs through a shell. A "job" represents a process group. Basic job control includes suspending, resuming, or terminating processes in the job/group. Advanced features involve sending signals to the job. This is particularly important in Unix due to its multiprocessing capabilities.

Chapter III: Goals
-------------------
The objective is to create a comprehensive job control daemon similar to supervisor. The program doesn't have to run as root or be a daemon. It will be started via a shell and provide a control shell to the user.

Chapter IV: General Instructions
-------------------------------
IV.1 Language Constraints
~~~~~~~~~~~~~~~~~~~~~~~~~
You can use any language and libraries for parsing configuration files and optional client/server implementation. Otherwise, stick to your language's standard library.

IV.2 Defense Session
~~~~~~~~~~~~~~~~~~~~~
During the defense session, demonstrate correct implementation of required features by running the program with a provided configuration file. The grader will test the program in various ways.

Chapter V: Mandatory Part
--------------------------
This project should be completed on a virtual machine. The program must start and maintain child processes, restarting them if needed. It should accurately track whether processes are alive or not. A configuration file, loaded at launch and reloadable with SIGHUP, will specify program details.

Chapter VI: Bonus Part
-----------------------
You're encouraged to implement extra features for points if they enhance the project. Ideas include privilege de-escalation on launch, client/server architecture, enhanced logging/alerts, and attaching/detaching supervised processes to/from the console.

Chapter VII: Appendix
----------------------
VII.1 Example Configuration File
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The configuration file for taskmaster could look like this:

.. code-block:: yaml

   programs:
     nginx:
       cmd: "/usr/local/bin/nginx -c /etc/nginx/test.conf"
       numprocs: 1
       umask: 022
       workingdir: /tmp
       autostart: true
       autorestart: unexpected
       exitcodes:
         - 0
         - 2
       startretries: 3
       starttime: 5
       stopsignal: TERM
       stoptime: 10
       stdout: /tmp/nginx.stdout
       stderr: /tmp/nginx.stderr
       env:
         STARTED_BY: taskmaster
         ANSWER: 42

     vogsphere:
       cmd: "/usr/local/bin/vogsphere-worker --no-prefork"
       numprocs: 8
       umask: 077
       workingdir: /tmp
       autostart: true
       autorestart: unexpected
       exitcodes: 0
       startretries: 3
       starttime: 5
       stopsignal: USR1
       stoptime: 10
       stdout: /tmp/vgsworker.stdout
       stderr: /tmp/vgsworker.stderr

VII.2 Trying out Supervisor
~~~~~~~~~~~~~~~~~~~~~~~~~~~
Supervisor, available on PyPI, offers similar functionality. Install it and interact with it using supervisorctl. Use it as inspiration for your project.

Chapter VIII: Submission and Peer Correction
--------------------------------------------
Submit your work on your Git repository as usual. Good luck and don't forget your author file!

