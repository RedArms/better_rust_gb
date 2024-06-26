
fn main() {

    let CPU = LR35902::init();

}

enum FLAG {
    Zero,
    NSub,
    HalfCarry,
    Carry,
    All
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
                        sp: 0xFFFE, pc: 0x100,
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

    fn set_BC(&mut self,value:u16){
        self.b = (value>>8) as u8;
        self.c = value as u8
    }

    fn set_DE(&mut self,value:u16){
        self.d = (value>>8) as u8;
        self.e = value as u8
    }
    fn set_HL(&mut self,value:u16){
        self.h = (value>>8) as u8;
        self.l = value as u8
    }

    fn get_next_byte(&mut self) -> u8 {
        return self.ram[(self.pc + 1) as usize];
    }

    fn get_next_two_bytes(&mut self) -> u16 {
        return self.ram[(self.pc + 1) as usize] as u16 | (self.ram[(self.pc + 2) as usize] as u16)<<8
    }

    fn dec_HL(&mut self) {
        let tmp = self.get_HL() - 1;
        self.h = (tmp>>8) as u8;
        self.h = tmp as u8;
    }

    fn inc_HL(&mut self) {
        let tmp = self.get_HL() + 1;
        self.h = (tmp>>8) as u8;
        self.h = tmp as u8;

    }
    fn up_flag(&mut self,flag:FLAG) {
        match flag {
            FLAG::Zero=>{self.f |= 0b1000_0000}
            FLAG::NSub=>{self.f |= 0b0100_0000}
            FLAG::HalfCarry=>{self.f |= 0b0010_0000}
            FLAG::Carry=>{self.f |= 0b0001_0000}
            FLAG::All=>{self.f = 0b1111_0000}
            _=>{}
        }
    }

    fn down_flag(&mut self,flag:FLAG) {
        match flag {
            FLAG::Zero=>{self.f &= 0b0111_0000}
            FLAG::NSub=>{self.f &= 0b1011_0000}
            FLAG::HalfCarry=>{self.f &= 0b1101_0000}
            FLAG::Carry=>{self.f &= 0b1110_0000}
            FLAG::All=>{self.f = 0b000_0000}
            _=>{}
        }
    }

    fn get_flag(&mut self,flag:FLAG) -> bool {
        match flag {
            FLAG::Zero=>{return self.f & 0b1000_0000 > 0}
            FLAG::NSub=>{return self.f & 0b0100_0000 > 0}
            FLAG::HalfCarry=>{return self.f & 0b0010_0000 > 0}
            FLAG::Carry=>{return self.f & 0b0001_0000 > 0}
            FLAG::All=>{return self.f & 0b1111_0000 > 0}
            _=>{return false;}
        }
    }

    fn fetch_ram_HL(&mut self) -> u8 {
        return self.ram[self.get_HL() as usize];
    }

    fn add(&mut self,value:u8){
        let res = self.a as u16 + value as u16;
        self.a = res as u8;
        self.down_flag(FLAG::NSub);
        self.down_flag(FLAG::Zero);
        if res == 0 {self.up_flag(FLAG::Zero)}
        if ((self.a & 0x0F) + (value & 0x0F) & 0xF0 ) > 0 {self.up_flag(FLAG::HalfCarry)} //yea thats how i see it 
        if res & 0xFF00 > 0 {self.up_flag(FLAG::Carry)}
    }

    fn adc(&mut self,value:u8){
        let mut carry:u16 = 0;
        if self.get_flag(FLAG::Carry) {
            carry = 1;
        }
        let res = self.a as u16 + value as u16 + carry;
        self.a = res as u8;
        self.down_flag(FLAG::NSub);
        self.down_flag(FLAG::Zero);
        if res == 0 {self.up_flag(FLAG::Zero)}
        if ((self.a & 0x0F) + (value & 0x0F) & 0xF0 ) > 0 {self.up_flag(FLAG::HalfCarry)} //yea thats how i see it 
        if res & 0xFF00 > 0 {self.up_flag(FLAG::Carry)}
    }

    //sub
    //subc

    fn and(&mut self,value:u8){
        self.a = self.a & value;
        self.down_flag(FLAG::NSub);
        self.down_flag(FLAG::Carry);
        self.up_flag(FLAG::HalfCarry);
        if self.a == 0 {self.up_flag(FLAG::Zero)}
    }

    fn or(&mut self,value:u8){
        self.a = self.a | value;
        self.down_flag(FLAG::NSub);
        self.down_flag(FLAG::Carry);
        self.down_flag(FLAG::HalfCarry);
        if self.a == 0 {self.up_flag(FLAG::Zero)}
    }

    fn xor(&mut self,value:u8){
        self.a = self.a ^ value;
        self.down_flag(FLAG::NSub);
        self.down_flag(FLAG::Carry);
        self.down_flag(FLAG::HalfCarry);
        if self.a == 0 {self.up_flag(FLAG::Zero)}
    }
    
    fn execute(&mut self) {
        let opcode = self.ram[self.pc as usize];
        let next_byte = self.get_next_byte();
        let next_two_bytes = self.get_next_two_bytes();
        let fetch_ram_HL = self.fetch_ram_HL();

        match opcode {
            0x00=>{
                //NOP
            }

            //LD 8 bits Loads
            //LD nn,n
            0x06=> {self.b = next_byte} //LD B,n
            0x0E=> {self.c = next_byte} //LD C,n
            0x16=> {self.d = next_byte} //LD D,n
            0x1E=> {self.e = next_byte} //LD E,n
            0x26=> {self.h = next_byte} //LD H,n
            0x2E=> {self.l = next_byte} //LD L,n
            //LD r1,r2
            //LD A,r                                                       
            0x7F=>{self.a = self.a}/*LD A,A */          //LD B,r
            0x78=>{self.a = self.b}/*LD A,B */          0x40=>{self.b = self.b}//LD B,B
            0x79=>{self.a = self.c}/*LD A,C */          0x41=>{self.b = self.c}//LD B,C
            0x7A=>{self.a = self.d}/*LD A,D */          0x42=>{self.b = self.d}//LD B,D
            0x7B=>{self.a = self.e}/*LD A,E */          0x43=>{self.b = self.e}//LD B,E
            0x7C=>{self.a = self.h}/*LD A,H */          0x44=>{self.b = self.h}//LD B,H
            0x7D=>{self.a = self.l}/*LD A,L */          0x45=>{self.b = self.l}//LD B,L
            0x7E=>{self.a = fetch_ram_HL}/*LD A,(HL)*/  0x46=>{self.b = fetch_ram_HL}//LD B,(HL)
            //LD C,r                                    //LD D,r
            0x48=>{self.c = self.b}/*LD C,B */          0x50=>{self.d = self.b}//LD D,B  4
            0x49=>{self.c = self.c}/*LD C,C */          0x51=>{self.d = self.c}//LD D,C  4
            0x4A=>{self.c = self.d}/*LD C,D */          0x52=>{self.d = self.d}//LD D,D  4
            0x4B=>{self.c = self.e}/*LD C,E */          0x53=>{self.d = self.e}//LD D,E  4
            0x4C=>{self.c = self.h}/*LD C,H */          0x54=>{self.d = self.h}//LD D,H  4
            0x4D=>{self.c = self.l}/*LD C,L */          0x55=>{self.d = self.l}//LD D,L  4
            0x4E=>{self.c = fetch_ram_HL}/*LD C,(HL)*/  0x56=>{self.d = fetch_ram_HL}//LD D,(HL) 56 8  
            //LD E,r                                    //LD H,r
            0x58=>{self.e = self.b}/*LD E,B */          0x60=>{self.h = self.b} //LD H,B
            0x59=>{self.e = self.c}/*LD E,C */          0x61=>{self.h = self.c} //LD H,C
            0x5A=>{self.e = self.d}/*LD E,D */          0x62=>{self.h = self.d} //LD H,D
            0x5B=>{self.e = self.e}/*LD E,E */          0x63=>{self.h = self.e} //LD H,E
            0x5C=>{self.e = self.h}/*LD E,H */          0x64=>{self.h = self.h} //LD H,H
            0x5D=>{self.e = self.l}/*LD E,L */          0x65=>{self.h = self.l} //LD H,L
            0x5E=>{self.e = fetch_ram_HL}/*LD E,(HL)*/  0x66=>{self.h = fetch_ram_HL}//LD H,(HL) 66 8  

            //LD L,r                                    //LD (HL),r
            0x68=>{self.l = self.b}/*LD L,B */          0x70=>{self.ram[self.get_HL() as usize] = self.b} //LD (HL),B
            0x69=>{self.l = self.c}/*LD L,C */          0x71=>{self.ram[self.get_HL() as usize] = self.c} //LD (HL),C
            0x6A=>{self.l = self.d}/*LD L,D */          0x72=>{self.ram[self.get_HL() as usize] = self.d} //LD (HL),D
            0x6B=>{self.l = self.e}/*LD L,E */          0x73=>{self.ram[self.get_HL() as usize] = self.e} //LD (HL),E
            0x6C=>{self.l = self.h}/*LD L,H */          0x74=>{self.ram[self.get_HL() as usize] = self.h} //LD (HL),H
            0x6D=>{self.l = self.l}/*LD L,L */          0x75=>{self.ram[self.get_HL() as usize] = self.l} //LD (HL),L
            0x6E=>{self.l = fetch_ram_HL}/*LD L,(HL)*/  0x36=>{self.ram[self.get_HL() as usize] = next_byte} //LD (HL),n 12
            
            //LD r,A                
            0x7F=>{self.a = self.a}/*LD A,A*/           0xF2=>{self.a = self.ram[(0xFF00 + self.c as u16) as usize]}//LD A,(C)      
            0x47=>{self.b = self.a}/*LD B,A*/           0xE2=>{self.ram[(0xFF00 + self.c as u16) as usize] = self.a}//LD (C),A               
            0x4F=>{self.c = self.a}/*LD C,A*/                    
            0x57=>{self.d = self.a}/*LD D,A*/           0x3A=>{self.a = fetch_ram_HL;self.dec_HL()}//LD A,(HL-)              
            0x5F=>{self.e = self.a}/*LD E,A*/           0x32=>{self.ram[self.get_HL() as usize] = self.a;self.dec_HL()}//LD (HLD),A               
            0x67=>{self.h = self.a}/*LD H,A*/           0x2A=>{self.a = fetch_ram_HL;self.inc_HL()}//LD A,(HL-)               
            0x6F=>{self.l = self.a}/*LD L,A*/           0x22=>{self.ram[self.get_HL() as usize] = self.a;self.inc_HL()}//LD (HLD),A            
            
            
            0x02=>{self.ram[self.get_BC() as usize] = self.a}/*LD (BC),A*/ 
            0x12=>{self.ram[self.get_DE() as usize] = self.a}/*LD (DE),A*/ 0xE0=>{self.ram[(0xFF00 + next_byte as u16) as usize] = self.a}//LD (n),A
            0x77=>{self.ram[self.get_HL() as usize] = self.a}/*LD (HL),A*/ 0xF0=>{self.a = self.ram[(0xFF00 + next_byte as u16) as usize]}//LD A,(n)
            0xEA=>{self.ram[next_two_bytes as usize]= self.a}//LD (nn),A
            0x3E=>{}//LD #,A ??


            //LD 16 bits Loads
            //LD n,nn
            0x01=>{self.set_BC(next_two_bytes)}//LD BC,nn 
            0x11=>{self.set_DE(next_two_bytes)}//LD DE,nn 
            0x21=>{self.set_HL(next_two_bytes)}//LD HL,nn 
            0x31=>{self.sp  =  next_two_bytes }//LD SP,nn 

            0xF9=>{self.sp = self.get_HL()}//LD SP,HL 
            0xF8=>{todo!()}// LDHL SP,n
            0x08=>{self.ram[next_two_bytes as usize] = self.pc as u8;
                   self.ram[(next_two_bytes+1) as usize] = (self.pc<<8) as u8}//LD (nn),SP
            
            //PUSH nn
            0xF5=>{self.ram[(self.pc - 1) as usize] = self.a;  self.ram[(self.pc - 2) as usize] = self.f;  self.pc = self.pc - 2}//PUSH AF 
            0xC5=>{self.ram[(self.pc - 1) as usize] = self.b;  self.ram[(self.pc - 2) as usize] = self.c;  self.pc = self.pc - 2}//PUSH BC 
            0xD5=>{self.ram[(self.pc - 1) as usize] = self.d;  self.ram[(self.pc - 2) as usize] = self.e;  self.pc = self.pc - 2}//PUSH DE 
            0xE5=>{self.ram[(self.pc - 1) as usize] = self.h;  self.ram[(self.pc - 2) as usize] = self.l;  self.pc = self.pc - 2}//PUSH HL 
            
            //POP nn
            0xF1=>{self.f = self.ram[self.pc as usize];   self.a = self.ram[(self.pc + 1) as usize];  self.pc = self.pc + 2}//POP AF 
            0xC1=>{self.c = self.ram[self.pc as usize];   self.b = self.ram[(self.pc + 1) as usize];  self.pc = self.pc + 2}//POP BC 
            0xD1=>{self.e = self.ram[self.pc as usize];   self.d = self.ram[(self.pc + 1) as usize];  self.pc = self.pc + 2}//POP DE 
            0xE1=>{self.l = self.ram[self.pc as usize];   self.h = self.ram[(self.pc + 1) as usize];  self.pc = self.pc + 2}//POP HL 



            //8bits ALU
            //ADD A,n                                        //ADC A,n
            0x87=>{self.add(self.a)}/*ADD A,A */             0x8F=>{self.adc(self.a)}//ADC A,A 
            0x80=>{self.add(self.b)}/*ADD A,B */             0x88=>{self.adc(self.b)}//ADC A,B 
            0x81=>{self.add(self.c)}/*ADD A,C */             0x89=>{self.adc(self.c)}//ADC A,C 
            0x82=>{self.add(self.d)}/*ADD A,D */             0x8A=>{self.adc(self.d)}//ADC A,D 
            0x83=>{self.add(self.e)}/*ADD A,E */             0x8B=>{self.adc(self.e)}//ADC A,E 
            0x84=>{self.add(self.h)}/*ADD A,H */             0x8C=>{self.adc(self.h)}//ADC A,H 
            0x85=>{self.add(self.l)}/*ADD A,L */             0x8D=>{self.adc(self.l)}//ADC A,L 
            0x86=>{self.add(fetch_ram_HL)}/*ADD A,(HL)*/     0x8E=>{self.adc(fetch_ram_HL)}//ADC A,(HL)
            0xC6=>{self.add(next_byte)}/*ADD A,#*/           0xCE=>{self.adc(next_byte)}//ADC A,#

            //SUB
            //SBC
            
            //AND                                                    //OR
            0xA7=>{self.and(self.a)}/*AND A*/                        0xB7=>{self.or(self.a)}/*OR A*/
            0xA0=>{self.and(self.b)}/*AND B*/                        0xB0=>{self.or(self.b)}/*OR B*/
            0xA1=>{self.and(self.c)}/*AND C*/                        0xB1=>{self.or(self.c)}/*OR C*/
            0xA2=>{self.and(self.d)}/*AND D*/                        0xB2=>{self.or(self.d)}/*OR D*/
            0xA3=>{self.and(self.e)}/*AND E*/                        0xB3=>{self.or(self.e)}/*OR E*/
            0xA4=>{self.and(self.h)}/*AND H*/                        0xB4=>{self.or(self.h)}/*OR H*/
            0xA5=>{self.and(self.l)}/*AND L*/                        0xB5=>{self.or(self.l)}/*OR L*/
            0xA6=>{self.and(fetch_ram_HL)}/*AND (HL) A6*/            0xB6=>{self.or(fetch_ram_HL)}/*OR (HL)*/
            0xE6=>{self.and(next_byte)}/*AND # E6*/                  0xF6=>{self.or(next_byte)}/*OR #*/

            //XOR
            0xAF=>{self.xor(self.a)}/*XOR A*/
            0xA8=>{self.xor(self.b)}/*XOR B*/
            0xA9=>{self.xor(self.c)}/*XOR C*/
            0xAA=>{self.xor(self.d)}/*XOR D*/
            0xAB=>{self.xor(self.e)}/*XOR E*/
            0xAC=>{self.xor(self.h)}/*XOR H*/
            0xAD=>{self.xor(self.l)}/*XOR L*/
            0xAE=>{self.xor(fetch_ram_HL)}/*XOR (HL)*/
            0xEE=>{self.xor(next_byte)}/*XOR #*/
            //CP

            
            _=>{println!("Unknow opcode")}
        }
    }

}