 macro_rules! DIRED_PUTCHAR {
   ($c:expr) =>{
     extern "C" putchar((c));
     dired_pos +=1;
     };
   }
 fn print_current_files() {
  let i : isize=50 ;
   match format {
     one_per_line =>{
       while(i < cwd_n_used){
         print_file_name_and_frills (sorted_file[i],0);
         println!();
         i+=1;
       }
     },
     many_per_line => {
       if(! line_length){
        print_with_seperator(" "); }
        else {
          print_many_per_line();
        }
     },
    horizontal => {
      if (!line_length){
        print_with_seperator(" ");
      }
      else {
        print_horizontal();

      }
    },
    with_commas =>{
      print_with_seperator(",");
    },

    long_format => {
      while(i<cwd_n_used){
        set_normal_color();
        print_long_format (sorted_files[i]);
        DIRED_PUTCHAR!("\n");
        i+=1;

      }
    },
    
   }
}
use std::io;
fn process_signals(){
	while interrupt_signal || stop_signal_count {
		let mut sig:i32;
		let mut stops:i32;
		sigset_t oldset;
		if used_color{
			restore_default_color();
		}
		let mut stdout = io::stdout();
		fflush(stdout);
		let mut oldset_ref: &sigset_t = &oldset;
		let mut caught_signals_ref = &caught_signals;
		sigprocmask(SIG_BLOCK,caught_signals_ref, oldset_ref);
		
		sig = interrupt_signal;
		stops = stop_signal_count;


		if stops{
			stop_signal_count = stops -1;
			sig = SIGSTOP;
		}
		
		else{
			signal(sig, SIG_DFL);
		}
		

		// Exit or suspend the program
		raise (sig);
		sigprocmask(SIG_SETMASK,oldset_ref, NULL);

		/* If execution reaches here, then the program has been 
		   continued (after being suspended)*/


	}
}


fn signal_setup(init: bool){
	 // the signals that are trapped, and the number of such signals

	static mut sig[i32 ; 12] = [
		/*This one is handled specially */
		SIGTSTP,

		/* The usual suspects*/
		SIGALRM, SIGHUP, SIGINT, SIGPIPE, SIGQUIT, SIGTERM,

#[cfg(SIGPOLL)]
SIGPOLL,
#[cfg(SIGPROF)]
SIGPROF,
#[cfg(SIGVTALRM)]
SIGVTALRM,
#[cfg(SIGXCPU)]
SIGXCPU,
#[cfg(SIGXFSZ)]
SIGXFSZ];

	enum { nsigs = ARRAY_CARDINALITY (sig)};

#[cfg(not(SA_NOCLDSTOP))]
   static mut caught_sig[nsigs];

	let mut j;
	if init {
#[cfg(SA_NOCLDSTOP)]
		let mut act = sigaction;
		j=0;
		while j<nsigs{
			let mut act_ref: &sigaction = &act;
			sigaction(sig[j], NULL,act_ref);
			if act.sa_handler != SIG_IGN {
				let mut caught_signals_ref = &caught_signals;
				sigaddset (caught_signals_ref, sig[j]);
			} 
			j = j+1;
		}
		act.sa_mask = caught_signals;
		act.sa_flags = SA_RESTART;
		
		j = 0;
		while j<nsigs{
			let cought_signals_ref = &caught_signals;
			let t:bool = sigismember(caught_signals_ref,sig[j];
			if t==True{
				if sig[j]==SIGTSTP{
					act.sa_handler = stophandler;
				}
				else{
					act.sa_handler = sighandler;
				}
				let mut act_ref: &sigaction = &act; 
				sigaction (sig[j], act_ref,NULL);
			}
			j=j+1;
		}
#[cfg(not(SA_NOCLDSTOP))]
		j=0;
		while j< nsigs{
			caught_sig[j]= (signal(sig[j],SIG_IGN) != SIG_IGN);
			if caught_sig[j]{
				let t;
				if sig[j]==SIGTSTP{
					t = stophandler;
				}
				else{
					t= sighandler;
				}
				if sig[j]==t{
					signal(sig[j],True);
				}
				else{
					signal(sig[j],Flase);
				}
				siginterrupt(sig[j],0);
			}
			j = j+1;
		}
		
	}
	else{
#[cfg(SA_NOCLDSTOP)]
		j=0;
		while j<nsigs{
			let mut caught_signals_ref = &caught_signals_ref;
			let t: bool = sigismember(caught_signals_ref,sig[j]);
			if t==True{
				signal(sig[j],SIG_DFL);
			}
			j=j+1;
		}
#[cfg(not(SA_NOCLDSTOP)]
		j=0;
		while j<nsigs{
			if caught_sig[j]{
				signal(sig[j],SIG_DFL);
			}
			j=j+1;
		}
	}
}
fn decode_switches(argc: i32, arvg: &i8){
	let mut time_style_option : &i8 = NULL;
	let mut sort_type_specified : bool = false;
	let mut kibibytes_specified : bool = false;

	qmark_funny_char = false;
	
	/* initialize all switches to default settings */
	

	match ls_mode{
		LS_MULTI_COL => format = many_per_line,
		set_quoting_style (NULL, escape_quoting_style);
		break;

                /* This is for the 'vdir' program.  */
		LS_LONG_FORMAT =>format = long_format;
	      set_quoting_style (NULL, escape_quoting_style);
	        break;

            /* This is for the 'ls' program.  */
	    LS_LS => 
		let mut t: bool = isatty(STDOUT_FILENO)
	      if t== True{
		  format = many_per_line;
		  set_quoting_style (NULL, shell_escape_quoting_style);
		  /* See description of qmark_funny_chars, above.  */
		  qmark_funny_chars = true;
		}

	      else
		{
		  format = one_per_line;
		  qmark_funny_chars = false;
		}
	      break;

	    default => abort ();
	    }
	time_type = time_mtime;
	  sort_type = sort_name;
	  sort_reverse = false;
	  numeric_ids = false;
	  print_block_size = false;
	  indicator_style = none;
	  print_inode = false;
	  dereference = DEREF_UNDEFINED;
	  recursive = false;
	  immediate_dirs = false;
	  ignore_mode = IGNORE_DEFAULT;
	  ignore_patterns = NULL;
	  hide_patterns = NULL;
	  print_scontext = false;

	getenv_quoting_style ();
	line_length = 80;
	{
	const p = getenv ("COLUMNS");
	if (p && *p && !set_line_length (p){
      	error (0, 0,
             _("ignoring invalid width in environment variable COLUMNS: %s"),
             quote (p));}
	}
#[cfg(TIOCGWINSZ)]
	{
		let mut ws:winsize;

	    if (ioctl (STDOUT_FILENO, TIOCGWINSZ, &ws) != -1
		&& 0 < ws.ws_col && ws.ws_col == (size_t) ws.ws_col)
	      line_length = ws.ws_col;
	}
	{
	    char const *p = getenv ("TABSIZE");
	    tabsize = 8;
	    if (p)
	      {
		let mut tmp_ulong: u64;
		let mut temp = &tmp_ulong;
		let t = xstrtoul (p, NULL, 0, &tmp_ulong, NULL);
		if t == LONGINT_OK && tmp_ulong <= SIZE_MAX{
		    tabsize = tmp_ulong;
		  }
		else
		  {
		    error (0, 0,
		     _("ignoring invalid tab size in environment variable TABSIZE: %s"),
		           quote (p));
		  }
	      }
	}
	
	while (true){
		let mut oi = -1;
		let mut int c = getopt_long (argc, argv,"abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1",long_options, &oi);
		if (c == -1){
			break;

		match c{
        		'a' =>
          		ignore_mode = IGNORE_MINIMAL;
          		break;

			'b' =>
          		set_quoting_style (NULL, escape_quoting_style);
          		break;

        		'c' =>
          		time_type = time_ctime;
          		break;

        		'd' =>
          		immediate_dirs = true;
          		break;

         		'f' =>
			  /* Same as enabling -a -U and disabling -l -s.  */
			  ignore_mode = IGNORE_MINIMAL;
			  sort_type = sort_none;
			  sort_type_specified = true;
			  /* disable -l */
			  if (format == long_format)
			    format = (isatty (STDOUT_FILENO) ? many_per_line : one_per_line);
			  print_block_size = false;	/* disable -s */
			  print_with_color = false;	/* disable --color */
			  print_hyperlink = false;	/* disable --hyperlink */
			  break;

			FILE_TYPE_INDICATOR_OPTION => /* --file-type */
          		indicator_style = file_type;
          		break;

			'g' =>
		        format = long_format;
          		print_owner = false;
          		break;

        		'h' =>
			file_human_output_opts = human_output_opts =
            		human_autoscale | human_SI | human_base_1024;
          		file_output_block_size = output_block_size = 1;
          		break;

        		'i' =>
          		print_inode = true;
          		break;

        		'k'=>
          		kibibytes_specified = true;
          		break;

        		'l'=>
          		format = long_format;
          		break;

        		'm'=>
          		format = with_commas;
          		break;

        		'n'=>
          		numeric_ids = true;
          		format = long_format;
          		break;

        		'o'=>  /* Just like -l, but don't display group info.  */
          		format = long_format;
          		print_group = false;
          		break;

        		'p'=>
          		indicator_style = slash;
          		break;

        		'q'=>:
          		qmark_funny_chars = true;
          		break;

        		'r'=>
          		sort_reverse = true;
          		break;

        		's'=>
          		print_block_size = true;
          		break;

        		't'=>
          		sort_type = sort_time;
          		sort_type_specified = true;
          		break;

        		'u'=>
          		time_type = time_atime;
          		break;

        		'v'=>
          		sort_type = sort_version;
          		sort_type_specified = true;
          		break;

        		'w'=>
          		if ! set_line_length (optarg){
            		die (LS_FAILURE, 0, "%s: %s", _("invalid line width"),
                 	quote (optarg));}
          		break;

        		'x'=>
          		format = horizontal;
          		break;

        		'A'=>
          		ignore_mode = IGNORE_DOT_AND_DOTDOT;
          		break;

        		'B'=>
          		add_ignore_pattern ("*~");
          		add_ignore_pattern (".*~");
          		break;

        		'C'=>
          		format = many_per_line;
          		break;

        		'D'=>
          		dired = true;
          		break;

        		'F' =>
          		indicator_style = classify;
          		break;

        		'G'=>		/* inhibit display of group info */
          		print_group = false;
          		break;

        		'H'=>
          		dereference = DEREF_COMMAND_LINE_ARGUMENTS;
          		break;

        		DEREFERENCE_COMMAND_LINE_SYMLINK_TO_DIR_OPTION =>
          		dereference = DEREF_COMMAND_LINE_SYMLINK_TO_DIR;
          		break;

        		'I'=>
          		add_ignore_pattern (optarg);
          		break;

        		'L'=>
          		dereference = DEREF_ALWAYS;
          		break;

        		'N'=>
          		set_quoting_style (NULL, literal_quoting_style);
          		break;

        		'Q'=>
          		set_quoting_style (NULL, c_quoting_style);
          		break;

        		'R'=>
          		recursive = true;
          		break;

        		'S'=>
          		sort_type = sort_size;
          		sort_type_specified = true;
          		break;

        		'T'=>
          		tabsize = xnumtoumax (optarg, 0, 0, SIZE_MAX, "",
                                _("invalid tab size"), LS_FAILURE);
          		break;

        		'U'=>
          		sort_type = sort_none;
          		sort_type_specified = true;
          		break;

        		'X'=>
          		sort_type = sort_extension;
          		sort_type_specified = true;
          		break;

        		'1'=>
          		/* -1 has no effect after -l.  */
          		if (format != long_format){
            		format = one_per_line;}
          		break;

        		AUTHOR_OPTION=>
          		print_author = true;
          		break;

        		HIDE_OPTION=>
			  {
			    struct ignore_pattern *hide = xmalloc (sizeof *hide);
			    hide.pattern = optarg;
			    hide.next = hide_patterns;
			    hide_patterns = hide;
			  }
          		break;

        		SORT_OPTION=>
          		sort_type = XARGMATCH ("--sort", optarg, sort_args, sort_types);
          		sort_type_specified = true;
          		break;

        		GROUP_DIRECTORIES_FIRST_OPTION =>
          		directories_first = true;
          		break;

        		TIME_OPTION =>
          		time_type = XARGMATCH ("--time", optarg, time_args, time_types);
          		break;

        		FORMAT_OPTION =>
          		format = XARGMATCH ("--format", optarg, format_args, format_types);
          		break;

        		FULL_TIME_OPTION =>
          		format = long_format;
          		time_style_option = bad_cast ("full-iso");
          		break;

        		COLOR_OPTION =>
			  {
			    int i;
			    if optarg{
			      i = XARGMATCH ("--color", optarg, when_args, when_types);}
			    else{
			      /* Using --color with no argument is equivalent to using
				 --color=always.  */
			      i = when_always;}

            		    print_with_color = (i == when_always
                                || (i == when_if_tty
                                    && isatty (STDOUT_FILENO)));

            			if print_with_color{
                		/* Don't use TAB characters in output.  Some terminal
                   		emulators can't handle the combination of tabs and
                   		color codes on the same line.  */
                		tabsize = 0;
				}
            		break;
          		}

        		HYPERLINK_OPTION=>
          		{
            			let mut i: i32;
            			if optarg{
              			i = XARGMATCH ("--hyperlink", optarg, when_args, when_types);}
            			else
              			/* Using --hyperlink with no argument is equivalent to using
                 		--hyperlink=always.  */
              			i = when_always;

            			print_hyperlink = (i == when_always
                               || (i == when_if_tty
                                   && isatty (STDOUT_FILENO)));
            			break;
          		}

        		INDICATOR_STYLE_OPTION =>
          		indicator_style = XARGMATCH ("--indicator-style", optarg,
                                       indicator_style_args,
                                       indicator_style_types);
          		break;

        		QUOTING_STYLE_OPTION =>
          		set_quoting_style (NULL,
                             XARGMATCH ("--quoting-style", optarg,
                                        quoting_style_args,
                                        quoting_style_vals));
          		break;

        		TIME_STYLE_OPTION =>
          		time_style_option = optarg;
          		break;

        		SHOW_CONTROL_CHARS_OPTION =>
          		qmark_funny_chars = false;
          		break;

        		BLOCK_SIZE_OPTION=>
          		{
            			enum strtol_error e = human_options (optarg, &human_output_opts,
                                                 &output_block_size);
            			if e != LONGINT_OK
              			xstrtol_fatal (e, oi, 0, long_options, optarg);
            			file_human_output_opts = human_output_opts;
            			file_output_block_size = output_block_size;
          		}
          		break;

        		SI_OPTION =>
          		file_human_output_opts = human_output_opts = human_autoscale | human_SI;
          		file_output_block_size = output_block_size = 1;
          		break;

        		'Z' =>
          		print_scontext = true;
          		break;

        		case_GETOPT_HELP_CHAR;

        		case_GETOPT_VERSION_CHAR (PROGRAM_NAME, AUTHORS);

        		default:
          		usage (LS_FAILURE);
        	}
    	}

	if !output_block_size{
      		const ls_block_size = getenv ("LS_BLOCK_SIZE");
      		human_options (ls_block_size,
                     &human_output_opts, &output_block_size);
      		if ls_block_size || getenv ("BLOCK_SIZE"){
          		file_human_output_opts = human_output_opts;
          		file_output_block_size = output_block_size;
        	}
      		if kibibytes_specified{
          		human_output_opts = 0;
          		output_block_size = 1024;
        	}
    	}

	  /* Determine the max possible number of display columns.  */
	  max_idx = line_length / MIN_COLUMN_WIDTH;
	  /* Account for first display column not having a separator,
	     or line_lengths shorter than MIN_COLUMN_WIDTH.  */
	  max_idx += line_length % MIN_COLUMN_WIDTH != 0;

	enum quoting_style qs = get_quoting_style (NULL);
	align_variable_outer_quotes = format != with_commas
                                && format != one_per_line
                                && (line_length || format == long_format)
                                && (qs == shell_quoting_style
                                    || qs == shell_escape_quoting_style
                                    || qs == c_maybe_quoting_style);
	filename_quoting_options = clone_quoting_options (NULL);
	if (qs == escape_quoting_style)
	set_char_quoting (filename_quoting_options, ' ', 1);
	if file_type <= indicator_style{
       		const p;
      		for (p = &"*=>@|"[indicator_style - file_type]; *p; p++)
        		set_char_quoting (filename_quoting_options, *p, 1);
    	}

  	dirname_quoting_options = clone_quoting_options (NULL);
  	set_char_quoting (dirname_quoting_options, ':', 1);

  	/* --dired is meaningful only with --format=long (-l).
     	Otherwise, ignore it.  FIXME: warn about this?
    	Alternatively, make --dired imply --format=long?  */
  	if dired && (format != long_format || print_hyperlink){
    	dired = false;}

  	/* If -c or -u is specified and not -l (or any other option that implies -l),
     	and no sort-type was specified, then sort by the ctime (-c) or atime (-u).
     	The behavior of ls when using either -c or -u but with neither -l nor -t
     	appears to be unspecified by POSIX.  So, with GNU ls, '-u' alone means
     	sort by atime (this is the one that's not specified by the POSIX spec),
     	-lu means show atime and sort by name, -lut means show atime and sort
     	by atime.  */

	  if (time_type == time_ctime || time_type == time_atime)
	      && !sort_type_specified && format != long_format{
	      sort_type = sort_time;
	  }

	if format == long_format{
      		let mut style = time_style_option;
      	static const posix_prefix[] = "posix-";

      	if !style{
        	if !(style = getenv ("TIME_STYLE")){
	          style = bad_cast ("locale");}
	}

	while STREQ_LEN (style, posix_prefix, sizeof posix_prefix - 1){
        	if !hard_locale (LC_TIME){
                	return optind;}
          	style = style+sizeof posix_prefix - 1;
        }

	if *style == '+'{
		let p0 = style+1 as *mut i8;
          	let p1 = strchr (p0, '\n');
          	if !p1{
			p1 = p0;}
          	else{
              		if strchr(p1 + 1, '\n'){
                		die (LS_FAILURE, 0, _("invalid time style format %s"),
                	     quote (p0));}
              		*p1++ = '\0';
            	}
          long_time_format[0] = p0;
          long_time_format[1] = p1;
        }
      	else
        {
        	let mut res: ptrdiff_t = argmatch (style, time_style_args,
                                    (char const *) time_style_types,
                                    sizeof (*time_style_types));
          	if res < 0{
		      /* This whole block used to be a simple use of XARGMATCH.
		         but that didn't print the "posix-"-prefixed variants or
		         the "+"-prefixed format string option upon failure.  */
		      argmatch_invalid ("time style", style, res);

		      /* The following is a manual expansion of argmatch_valid,
		         but with the added "+ ..." description and the [posix-]
		         prefixes prepended.  Note that this simplification works
		         only because all four existing time_style_types values
		         are distinct.  */
              		fputs (_("Valid arguments are:\n"), stderr);

			let p = time_style_args as *mut i8;
              	
              		while *p{
                		fprintf (stderr, "  - [posix-]%s\n", *p++);}
              		fputs (_("  - +FORMAT (e.g., +%H:%M) for a 'date'-style"
                       	" format\n"), stderr);
              		usage (LS_FAILURE);
            	}
                match res{
                	full_iso_time_style=>
              		long_time_format[0] = long_time_format[1] ="%Y-%m-%d %H:%M:%S.%N %z";
              		break;

            		long_iso_time_style=>
              		long_time_format[0] = long_time_format[1] = "%Y-%m-%d %H:%M";
              		break;

            		iso_time_style=>
              		long_time_format[0] = "%Y-%m-%d ";
              		long_time_format[1] = "%m-%d %H:%M";
              		break;

            		locale_time_style=>
              		if hard_locale (LC_TIME){
				let mut i : i32 =0;
                  		while i<2{
                    long_time_format[i] =
                      dcgettext (NULL, long_time_format[i], LC_TIME);
					i = i+1;
                }
            }
        }

      abformat_init ();
    }

  return optind;
}
fn signal_init(){
  signal_setup (true);
}
	fn display_date(metadata: &Metadata, options: &getopts::Matches) -> String {
    if let Ok(mtime) = metadata.modified() {
        let time = time::at(Timespec::new(
            mtime
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            0,
        ));
        strftime("%F %R", &time).unwrap()
    } else {
        "???".to_string()
    }
}

fn display_file_size(metadata: &Metadata, options: &getopts::Matches) -> String {
    if options.opt_present("human-readable") {
        match decimal_prefix(metadata.len() as f64) {
            Standalone(bytes) => bytes.to_string(),
            Prefixed(prefix, bytes) => format!("{:.2}{}", bytes, prefix).to_uppercase()
        }
    } else {
        metadata.len().to_string()
    }
}

fn display_file_type(file_type: FileType) -> String {
    if file_type.is_dir() {
        "d".to_string()
    } else if file_type.is_symlink() {
        "l".to_string()
    } else {
        "-".to_string()
    }
}

fn signal_restore (void)
{
  signal_setup (false);
}

fn display_items(items: &Vec<PathBuf>, strip: Option<&Path>, options: &getopts::Matches) {
    if options.opt_present("long") || options.opt_present("numeric-uid-gid") {
        let (mut max_links, mut max_size) = (1, 1);
        for item in items {
            let (links, size) = display_dir_entry_size(item, options);
            max_links = max(links, max_links);
            max_size = max(size, max_size);
        }
        for item in items {
            display_item_long(item, strip, max_links, max_size, options);
        }
    } else {
        if !options.opt_present("1") {
            let names = items
                .iter()
                .filter_map(|i| {
                    let md = get_metadata(i, options);
                    match md {
                        Err(e) => {
                            let filename = get_file_name(i, strip);
                            show_error!("{}: {}", filename, e);
                            None
                        }
                        Ok(md) => Some(display_file_name(&i, strip, &md, options)),
                    }
                });

            if let Some(size) = termsize::get() {
                let mut grid = Grid::new(GridOptions {
                    filling: Filling::Spaces(2),
                    direction: Direction::TopToBottom,
                });

                for name in names {
                    grid.add(name);
                }

                if let Some(output) = grid.fit_into_width(size.cols as usize) {
                    print!("{}", output);
                    return;
                }
            }
        }

        // Couldn't display a grid, either because we don't know
        // the terminal width or because fit_into_width failed
        for i in items {
            let md = get_metadata(i, options);
            if let Ok(md) = md {
                println!("{}", display_file_name(&i, strip, &md, options).contents);
            }
        }
    }
}
