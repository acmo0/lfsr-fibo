use std::collections::VecDeque;

/// Structure representing a Fibonacci LFSR
#[derive(Debug, Clone, Hash)]
pub struct Lfsr {
    state: VecDeque<u8>,
    taps: Vec<u8>,
}


impl Lfsr {
    /// Return a new LFSR from given taps.
    ///
    /// # Example
    ///
    /// If the fibonacci polynomial representing your LFSR is x^19 + x^13 + x^2 + 1,
    /// then you could get that LFSR using :
    ///
    /// ```
    /// let lfsr = LFSR::new(vec![19, 13, 2]);
    /// ```
    pub fn new(taps: Vec<u8>) -> Lfsr {
        Lfsr {
            state: VecDeque::new(),
            taps: taps.into_iter().map(|t| t-1).collect(),
        }
    }
    
    /// Set the key of the LFSR
    ///
    /// # Example
    /// 
    /// Let say that your key is [0, 1, 1, 0, 0, 0, 1], then :
    ///
    /// ```
    /// let mut lfsr = LFSR::new(vec![7, 5, 2]);
    /// let key: VecDeque<u8> = VecDeque::from([0, 1, 1, 0, 0, 0, 1]);
    /// 
    /// lfsr.set_key(key);
    /// ```
    pub fn set_key(&mut self, key:VecDeque<u8>) {
        self.state = key;
    }

    /// Internal function that compute the carry of the LFSR
    /// i.e the XOR of all the state elements referenced in the taps.
    fn _carry(&self) -> u8{
        self.taps.iter().map(|&i| self.state[i as usize]).sum::<u8>() & 1
    }

    /// Clock the LFSR once and output the bit as an u8.
    ///
    /// # Example
     /// Let say that your key is [0, 1, 1, 0, 0, 0, 1], then :
    ///
    /// ```
    /// let mut lfsr = LFSR::new(vec![7, 5, 2]);
    /// let key: VecDeque<u8> = VecDeque::from([0, 1, 1, 0, 0, 0, 1]);
    /// 
    /// lfsr.set_key(key);
    /// println!("First bit is : {:?}", lfsr.clock());
    /// ```
    pub fn clock(&mut self) -> u8 {
        self.state.push_front(self._carry());
        self.state.pop_back().unwrap()
    }
}