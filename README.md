# wsync
Rsync "wrapper" for making backup scripts easier to write.\
I don't really know how to program, so if you see something wrong, please make a PR or open a issue.

## Usage

Syncing folder or file:
```
/home/user/some_folder -> /home/user/backup/some_folder
```
The rsync command for this is ```rsync -av src dest --delete```


Run command:
```
$(git add. && git commit -m "backup $(date +%s)" && git push)
```

Import other file:
```
-import dotfiles.sync
```

Print message:
```
<-- Some message -->
```

Comment:
```
// comment
```

Then just run the program with: ```wsync file.sync```


## Building
```
git clone https://github.com/dot1nt/wsync.git && cd wsync
cargo build --release
sudo cp target/release/wsync /usr/local/bin/wsync
```

