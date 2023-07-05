// fn build_dir_structure_until_complete(
//     dir: Rc<RefCell<Dir>>,
//     lines: &mut Peekable<std::str::Lines>,
// ) {
//     while let Some(_) = lines.peek() {
//         println!("some!");
//         build_dir_structure(Rc::clone(&dir), lines);
//     }
// }

// fn build_dir_structure(
//     dir: Rc<RefCell<Dir>>,
//     lines: &mut Peekable<std::str::Lines>,
// ) -> bool {
//     {
//         let mut value = VALUE.lock().unwrap();
//         println!("{}: {}", value, dir.borrow().name);
//         *value += 1;
//     }
//     loop {
//         let line = lines.next();
//         if let None = line {
//             return false;
//         }
//         let line = line.unwrap();
//         println!("----- COMMAND [ {} ]", line);
//         if &line[2..4] == "cd" {
//             // is cd
//             // build child
//             let name = &line[5..];

//             if name == ".." {
//                 // go to parent dir
//                 println!("returning to parent dir...");
//                 return false;
//             }

//             if name == "/" {
//                 // go to "/"
//                 println!("returning to /...");
//                 return true;
//             }

//             // only add a child_dir if it has not been found before
//             let child_dir = dir
//                 .borrow()
//                 .child_dirs
//                 .iter()
//                 .filter(|child| child.borrow().name == name)
//                 .next()
//                 .map(|c| Rc::clone(c));
//             let child_dir = child_dir.unwrap_or({
//                 let child = Rc::new(RefCell::new(Dir::new(name)));
//                 dir.borrow_mut().child_dirs.push(Rc::clone(&child));
//                 child
//             });

//             println!("entering child_dir: {}", child_dir.borrow().name);
//             return build_dir_structure(Rc::clone(&child_dir), lines);
//         } else {
//             // is ls
//             // build current
//             // turns out, when ls happens we only have to capture the files
//             // since we are guaranteed to get a 'cd <dir>' for the dirs (right?)
//             if dir.borrow().files.len() != 0 {
//                 break;
//             }
//             loop {
//                 let line = lines.peek();
//                 if let None = line {
//                     println!("done with text");
//                     return true;
//                 }
//                 if &line.unwrap()[..1] == "$" {
//                     // new command, break out before consuming the line
//                     println!("next command");
//                     break;
//                 }

//                 // consume, definitely a file or dir
//                 let line = lines.next().unwrap();
//                 let parts = line.split_whitespace().collect::<Vec<&str>>();

//                 // only need to add files once, so we can just check files.len() == 0
//                 if parts[0].chars().all(|c| c.is_numeric()) {
//                     // we found a file
//                     println!(
//                         "add file {} of size {} to {}",
//                         parts[1],
//                         parts[0],
//                         dir.borrow().name
//                     );
//                     dir.borrow_mut()
//                         .files
//                         .push(parts[0].parse::<i64>().unwrap());
//                 } else {
//                     // dir ... do nothing
//                 }
//             }
//         }
//     }
//     *VALUE.lock().unwrap() += 1;
//     return false;
// }
