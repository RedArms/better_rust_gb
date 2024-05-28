fn main() {

    let CPU = LR35902::init();

}

struct LR35902 {
a:u8,f:u8,
b:u8,c:u8,
d:u8,e:u8,
h:u8,l:u8,

sp:u16,
pc:u16,

ram:[u8;0xFFFF+1]
}

impl LR35902 {
    fn init() -> LR35902 {
        return LR35902{ a: 0, f: 0, 
                        b: 0, c: 0, 
                        d: 0, e: 0,
                        h: 0, l: 0,
                        sp: 0, pc: 0x100,
                        ram: [0;0xFFFF+1]
                    };
    }

    fn get_BC(&mut self) -> u16 {
        return (self.b as u16) << 8 + self.c;
    }

    fn get_DE(&mut self) -> u16 {
        return (self.d as u16) << 8 + self.e;
    }
    fn get_HL(&mut self) -> u16 {
        return (self.h as u16) << 8 + self.l;
    }
    fn get_next_byte(&mut self) -> u8 {
        return self.ram[(self.pc + 1) as usize];
    }
    
    fn execute(&mut self) {
        let opcode = self.ram[self.pc as usize];

        match opcode {
            0x00=>{
                //NOP
            }

            //LD nn,n
            0x06=> {self.b = self.get_next_byte()} //LD B,n
            0x0E=> {self.c = self.get_next_byte()} //LD C,n
            0x16=> {self.d = self.get_next_byte()} //LD D,n
            0x1E=> {self.e = self.get_next_byte()} //LD E,n
            0x26=> {self.h = self.get_next_byte()} //LD H,n
            0x2E=> {self.l = self.get_next_byte()} //LD L,n

            //LD r1,r2
            0x7F=>{self.a = self.a}/*LD A,A */ 
            0x78=>{self.a = self.b}/*LD A,B */    0x40=>{self.b = self.b}//LD B,B
            0x79=>{self.a = self.c}/*LD A,C */    0x41=>{self.b = self.c}//LD B,C
            0x7A=>{self.a = self.d}/*LD A,D */    0x42=>{self.b = self.d}//LD B,D
            0x7B=>{self.a = self.e}/*LD A,E */    0x43=>{self.b = self.e}//LD B,E
            0x7C=>{self.a = self.h}/*LD A,H */    0x44=>{self.b = self.h}//LD B,H
            0x7D=>{self.a = self.l}/*LD A,L */    0x45=>{self.b = self.l}//LD B,L
            0x7E=>{}/*LD A,(HL)*/  0x46=>{}//LD B,(HL)
                  
            0x48=>{}//LD C,B  4
            0x49=>{}//LD C,C  4
            0x4A=>{}//LD C,D  4
            0x4B=>{}//LD C,E  4
            0x4C=>{}//LD C,H  4
            0x4D=>{}//LD C,L  4
            //LD C,(HL) 4E 8
//
            0x50=>{}//LD D,B  4
            0x51=>{}//LD D,C  4
            0x52=>{}//LD D,D  4
            0x53=>{}//LD D,E  4
            0x54=>{}//LD D,H  4
            0x55=>{}//LD D,L  4
            //LD D,(HL) 56 8
//
            0x58=>{}//LD E,B  4
            0x59=>{}//LD E,C  4
            0x5A=>{}//LD E,D  4
            0x5B=>{}//LD E,E  4
            0x5C=>{}//LD E,H  4
            0x5D=>{}//LD E,L  4
            //LD E,(HL) 5E 8
//
            0x60=>{} //LD H,B 4
            0x61=>{} //LD H,C 4
            0x62=>{} //LD H,D 4
            0x63=>{} //LD H,E 4
            0x64=>{} //LD H,H 4
            0x65=>{} //LD H,L 4
            //LD H,(HL) 66 8
//
            0x68=>{}//LD L,B  4
            0x69=>{}//LD L,C  4
            0x6A=>{}//LD L,D  4
            0x6B=>{}//LD L,E  4
            0x6C=>{}//LD L,H  4
            0x6D=>{}//LD L,L  4
            //LD L,(HL) 6E 8
            //

            0x70=>{} //LD (HL),B 8
            0x71=>{} //LD (HL),C 8
            0x72=>{} //LD (HL),D 8
            0x73=>{} //LD (HL),E 8
            0x74=>{} //LD (HL),H 8
            0x75=>{} //LD (HL),L 8
            0x36=>{} //LD (HL),n 12


            _=>{println!("Unknow opcode")}
        }
    }

}