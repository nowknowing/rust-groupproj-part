fn main() {
    fn a() -> (){
        {
            let y = 1;
            {   let a = 1;
                return y;}
            let x = 3;
            ..
            ..
            // No return
        } // Block  is of type <null>   ();
        return ();
    }
/*
    fn b() -> (){
        {
            let y = 1;
            return (); // has return
        } // Block  is of type <()> 
        return ();
    }

    fn c() -> (){
        let z = {
            let y = 1;
            return y;
            // No return
        }; // ExprStmt(BlockExpr(...)) ----> (); // Nothing; Block is of type <null>
        return ();
    }
    */
}
