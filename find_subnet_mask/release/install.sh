# check git 
if [ "$(git -v)" == "" ]; then
	echo "git not install. please install git and run again.";
	exit;
fi

# create directory for program
mkdir $HOME/.subnet
mkdir $HOME/.subnet/bin

git clone https://github.com/Arikato111/learn-rust-projects.git $HOME/.subnet/gi
cp $HOME/.subnet/gi/find_subnet_mask/release/bin/subnet $HOME/.subnet/bin
rm -rf $HOME/.subnet/gi

# check shell
if [ "${SHELL#*bash}" != "$SHELL" ]; then
  if [ -f "$HOME/.bashrc" ]; then
    DETECTED_PROFILE="$HOME/.bashrc"
  elif [ -f "$HOME/.bash_profile" ]; then
    DETECTED_PROFILE="$HOME/.bash_profile"
  fi
elif [ "${SHELL#*zsh}" != "$SHELL" ]; then
  if [ -f "$HOME/.zshrc" ]; then
    DETECTED_PROFILE="$HOME/.zshrc"
  elif [ -f "$HOME/.zprofile" ]; then
    DETECTED_PROFILE="$HOME/.zprofile"
  fi
fi

# set PATH to shell
echo export PATH=$PATH:$HOME/.subnet/bin >> $DETECTED_PROFILE;