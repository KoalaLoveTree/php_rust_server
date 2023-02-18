# PHP rust server lib

Library that adds function to execute web server that process requests with provided closure.

### How to install

Run following commands and module file will be generated and moved to php modules directory

```cargo build --release```

```phpize```

```make```

```make install```

After you need to open your ```php.ini``` file and add extension there:

```extension=/path/to/php/modules/php_rust_server.so```

### Doc

Right now only available functions is:

```start_server($closure)```

Where ```$closure``` is instance of ```Closure``` class.


## Examples

You can test module functionality simply by executing commands in your terminal:

```php example.php```

```example.php``` is in ```/``` directory of project.

Then simply open in your browser:

```http://127.0.0.1:3000/?first=x&second=y```