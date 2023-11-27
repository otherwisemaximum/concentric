pipeline{
    agent "any"

    stages{
        stage("cargo fmt"){
            steps{
                echo "========executing cargo fmt========"
                cargo fmt --check
                if [$? -eq 0]; then
                    exit 1
                fi
            }
        }
    }
}