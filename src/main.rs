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
            //LD A,r                                                       //LD B,r
            0x7F=>{self.a = self.a}/*LD A,A */ 
            0x78=>{self.a = self.b}/*LD A,B */                             0x40=>{self.b = self.b}//LD B,B
            0x79=>{self.a = self.c}/*LD A,C */                             0x41=>{self.b = self.c}//LD B,C
            0x7A=>{self.a = self.d}/*LD A,D */                             0x42=>{self.b = self.d}//LD B,D
            0x7B=>{self.a = self.e}/*LD A,E */                             0x43=>{self.b = self.e}//LD B,E
            0x7C=>{self.a = self.h}/*LD A,H */                             0x44=>{self.b = self.h}//LD B,H
            0x7D=>{self.a = self.l}/*LD A,L */                             0x45=>{self.b = self.l}//LD B,L
            0x7E=>{self.a = self.ram[self.get_HL() as usize]}/*LD A,(HL)*/ 0x46=>{self.b = self.ram[self.get_HL() as usize]}//LD B,(HL)
            //LD C,r                                                       //LD D,r
            0x48=>{self.c = self.b}/*LD C,B */                             0x50=>{self.d = self.b}//LD D,B  4
            0x49=>{self.c = self.c}/*LD C,C */                             0x51=>{self.d = self.c}//LD D,C  4
            0x4A=>{self.c = self.d}/*LD C,D */                             0x52=>{self.d = self.d}//LD D,D  4
            0x4B=>{self.c = self.e}/*LD C,E */                             0x53=>{self.d = self.e}//LD D,E  4
            0x4C=>{self.c = self.h}/*LD C,H */                             0x54=>{self.d = self.h}//LD D,H  4
            0x4D=>{self.c = self.l}/*LD C,L */                             0x55=>{self.d = self.l}//LD D,L  4
            0x4E=>{self.c = self.ram[self.get_HL() as usize]}/*LD C,(HL)*/ 0x56=>{self.d = self.ram[self.get_HL() as usize]}//LD D,(HL) 56 8  
            //LD E,r                                                       //LD H,r
            0x58=>{self.e = self.b}/*LD E,B */                             0x60=>{self.h = self.b} //LD H,B
            0x59=>{self.e = self.c}/*LD E,C */                             0x61=>{self.h = self.c} //LD H,C
            0x5A=>{self.e = self.d}/*LD E,D */                             0x62=>{self.h = self.d} //LD H,D
            0x5B=>{self.e = self.e}/*LD E,E */                             0x63=>{self.h = self.e} //LD H,E
            0x5C=>{self.e = self.h}/*LD E,H */                             0x64=>{self.h = self.h} //LD H,H
            0x5D=>{self.e = self.l}/*LD E,L */                             0x65=>{self.h = self.l} //LD H,L
            0x5E=>{self.e= self.ram[self.get_HL() as usize]}/*LD E,(HL)*/  0x66=>{self.h = self.ram[self.get_HL() as usize]}//LD H,(HL) 66 8  
            //LD L,r                                                       //LD (HL),r
            0x68=>{self.l = self.b}/*LD L,B */                             0x70=>{self.c = self.b} //LD (HL),B 8
            0x69=>{self.l = self.c}/*LD L,C */                             0x71=>{self.c = self.c} //LD (HL),C 8
            0x6A=>{self.l = self.d}/*LD L,D */                             0x72=>{self.c = self.d} //LD (HL),D 8
            0x6B=>{self.l = self.e}/*LD L,E */                             0x73=>{self.c = self.e} //LD (HL),E 8
            0x6C=>{self.l = self.h}/*LD L,H */                             0x74=>{self.c = self.h} //LD (HL),H 8
            0x6D=>{self.l = self.l}/*LD L,L */                             0x75=>{self.c = self.l} //LD (HL),L 8
            0x6E=>{self.l = self.ram[self.get_HL() as usize]}/*LD L,(HL)*/ 0x36=>{self.ram[self.get_HL() as usize] = self.get_next_byte()} //LD (HL),n 12

            //LD r,A
            0x7F=>{self.a = self.a}//LD A,A
            0x47=>{self.b = self.a}//LD B,A
            0x4F=>{self.c = self.a}//LD C,A
            0x57=>{self.d = self.a}//LD D,A
            0x5F=>{self.e = self.a}//LD E,A
            0x67=>{self.h = self.a}//LD H,A
            0x6F=>{self.l = self.a}//LD L,A
            0x02=>{self.ram[self.get_BC() as usize] = self.a}//LD (BC),A
            0x12=>{self.ram[self.get_DE() as usize] = self.a}//LD (DE),A
            0x77=>{self.ram[self.get_HL() as usize] = self.a}//LD (HL),A
            0xEA=>{}//LD (nn),A


            _=>{println!("Unknow opcode")}
        }
    }

}