// module imports
mod leontief;
mod multipliers;
mod influence;
mod linkages;
mod ghosh;
mod extraction;

use extendr_api::prelude::*;
use num_cpus;
use rayon::ThreadPoolBuilder;

#[extendr]
/// Sets max number of threads used by fio
/// 
/// @details
/// Calling this function sets a global limit of threads to Rayon crate, affecting
/// all computations that runs in parallel by default.
/// 
/// Default behaviour of Rayon is to use all available threads (including logical).
/// Setting to 1 will result in single threaded (sequencial) computations.
/// 
/// Initialization of the global thread pool happens exactly once.
/// Once started, the configuration cannot be changed in the current session.
/// If `set_max_threads()` is called again in the same session, it'll result
/// in an error.
/// 
/// @param max_threads Int.
/// Default is 0 (all threads available). 1 means single threaded.
/// 
/// @return
/// This functions does not return a value.
/// 
/// @examples
/// \dontrun{
///   intermediate_transactions <- matrix(c(1, 2, 3, 4, 5, 6, 7, 8, 9), 3, 3)
///   total_production <- matrix(c(100, 200, 300), 1, 3)
///   # instantiate iom object
///   my_iom <- fio::iom$new("test", intermediate_transactions, total_production)
///   # runs on only 2 threads
///   my_iom$set_max_threads(2L)
/// }

fn set_max_threads(max_threads: usize) {

  // If max_threads is 0, use the maximum number of available threads
  let num_threads = if max_threads == 0 {
      num_cpus::get()
  } else {
      max_threads
  };
  
  ThreadPoolBuilder::new()
  .num_threads(num_threads)
  .build_global()
  .unwrap();
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
  mod fio;
  use leontief;
  use multipliers;
  use influence;
  use linkages;
  use ghosh;
  use extraction;
  fn set_max_threads;
}
