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
