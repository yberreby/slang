// vim: set syntax=rust:

// Statically-allocated variables may be declared.
static count: i64;

// Code outside a function will be put in the object file as-is.
!count = 0;

// A struct is just a collection of data.
// No special alignment is done by default.
// The order of parameters is preserved.
struct linked_list {
  layout {
    // There is no "int" type, except (eventually) for C FFI.
    data: i64,
    // void pointer arithmetic is +1 = +1 byte
    next: *void
  }

  // This is the default value of an uninitialized value of type `linked_list`.
  default {
    data: 0xDEADBEEF,
    // null is a primitive constant
    next: null
  }
}


/*
`self.method(a,b,c)`
is an alias for

`Type::method(self, a, b, c)`
*/


impl linked_list {
  fn prepend(ll: *linked_list, new_data: i64) -> *linked_list {
    // This is syntactic sugar for declaration + assignment.
    let new_ll: *linked_list = ll_init(new_data);
    // expr->ident is an alias for (*expr).ident
    new_ll->next = ll;

    return new_ll;
  }
}


fn ll_init(data: i64) -> *linked_list {
  let ll: *linked_list; // This is a stack-allocated variable declaration.

  // The `allocate` keyword takes a *type* and reserves enough memory to store it,
  // returning a *properly typed* pointer to the newly reserved space.
  // 
  // A default memory layout MAY be specified to facilitate debugging.
  //
  // No constructor, or any other code, is run.
  // 
  // An assignment exclusively operates on local variables.
  //
  ll = allocate linked_list;

  (*ll).data = data;

  // A global variable may be accessed by prepending a bang to its name.
  !count = !count + 1;

  return ll;
}


// A function may have no return type, in which case it is a procedure.
fn ll_free(ll: *linked_list) {
  while (ll != NULL) {
    let next: linked_list* = (*ll).next;
    deallocate ll;
    ll = next;
  }
}


let ll: *linked_list = ll_init(1);

// "method" calls are just syntactic sugar.
// TODO explicit
ll = ll.prepend(3);
ll = ll.prepend(5);
ll = ll.prepend(7);



// while (ll) would NOT work.
// bool is a primitive type and implicit casting is forbidden.
while (ll != null) {
  std::print_i64(ll);
  std::print_newline();
}
