<br />
<p align="center">
  <a href="">
    <img src=".logo.png" alt="Logo" width="90" height="90">
  </a>

  <h1 align="center">Taskmaster</h1>

  <p align="center"><i>Unix-like process control system that facilitates the monitoring and management of long-running processes.</i>
  </p>
</p>

---

#### ToDo

- [ ] auto release when tag on Rust check->build->test->audit->release
- [ ] docker image build and publish on dockerhub and in release along with binaries when tag
- [X] parsing of the daemon config file in ini, yaml and json
- [ ] parsing of daemon cli
- [ ] enabling execution as daemon or not
- [ ] succeed make use taskmasterd with supervisorctl
- [ ] build core execution of programs and its management (use supervisorctl to run taskmasterd programs)


#### Containers

`docker run -p 8080:80 -d <nginx_supervisor>`

#### Usage

`cargo run --bin taskmasterd -- -c config/d/supervisord.[conf|yaml|json]`

#### Sources
- [How to Install and Manage Supervisor on Ubuntu and Debian VPS](https://www.digitalocean.com/community/tutorials/how-to-install-and-manage-supervisor-on-ubuntu-and-debian-vps)
- [Getting Started with Supervisor](https://blog.programster.org/getting-started-with-supervisor)
- [How to Use Supervisor in Linux](https://blog.knoldus.com/how-to-use-supervisor-in-linux/)
- [Managing Processes with Supervisor - In-Depth Tutorial](https://csjourney.com/managing-processes-with-supervisor-in-depth-tutorial/)
- [Utiliser Supervisor pour contrôler ses services applicatifs](https://www.elao.com/blog/infra/utiliser-supervisor-pour-controler-ses-services-applicatifs)
- [Systemd vs Supervisor](https://ege.dev/posts/systemd-vs-supervisor/)

#### Official Documentation
- [Supervisord](http://supervisord.org/)

#### Official Github Organization
- [Github/Supervisor](https://github.com/Supervisor)
