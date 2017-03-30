node('jenkins-slave-rust') {
    env.PATH = "${env.PATH}:${env.HOME}/.cargo/bin"
    env.CACHE_ROOT = "/home/shared/sda/cache"
    env.CARGO_HOME = "/home/shared/sda/cache/cargo"

    stage('Checkout') {
        checkout scm
        sh "git remote rm origin"
        sh "git remote add origin 'git@github.com:snipsco/sda.git'"
        sh "git config --global user.email 'jenkins@snips.ai'"
        sh "git config --global user.name 'Jenkins'"
    }

    stage('Hello') {
        sh "rustc --version"
        sh "cargo --version"
    }

    stage('Build and test Rust SDA') {
        parallel(
            'server': { sh "cd server; cargo build && cargo test" },
            'sdad': { sh "cd server-cli; cargo build" },
            'integration': { sh "cd integration-tests; cargo test && cargo test --features http" }
            'clis': { sh "./docs/simple-cli-example.sh" }
        )
    }

}
