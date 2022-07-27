	(
		cd ../../..
		git clone https://github.com/AcalaNetwork/Acala.git
		cd Acala
		make init
		make build-release
		./target/production/acala --version
	)