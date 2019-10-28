// RECORDATORIO: quitar el guión bajo antes de los parámetros después
// de implementar la función y quitar #[allow(dead_code)] cuando de que
// la función sea usada en otro lado.

extern crate num;
extern crate num_bigint;
extern crate rand;

use num::bigint::{ToBigUint, ToBigInt, BigUint,BigInt};
use num_bigint:: RandBigInt;

#[allow(dead_code)]
const MESSAGE_MAX_LENGTH: usize = 257;

const NUM_BITS: usize = 100;
const N_PRUEBAS : u64 =  300;

pub struct RSA {
    n: BigUint,
    e: BigUint,
    _d: BigUint,
}

// receives an integer and returns a BigInteger
pub fn big(n: i64) -> BigInt {
    return n.to_bigint().unwrap();
}

// receives an integer and returns a BigUInteger
pub fn ubig(n: u64) -> BigUint {
    return n.to_biguint().unwrap();
}

impl RSA {

    #[allow(dead_code)]
    pub fn new() -> RSA{
        let p = RSA::generar_primo();
        let q = RSA::generar_primo();
        let n1 = p.clone()*q.clone();
        let phi = (p.clone()-ubig(1))*(q.clone()-ubig(1));
        let (e1, d1) = RSA::generar_ed(phi.clone());
            
        RSA {
            n: n1,
            e: e1,
            _d: d1
        }
    }   
        
    // recibe dos enteros a y b
    // devuelve d,x,y donde:
    // d = mcd(a,b)
    // x = a^-1 mod b en caso de d=1
    // y = b^-1 mod a en caso de d=1
    // NOTA: x y y pueden ser negativos
    fn euclides_extendido(a:BigInt , b: BigInt) -> (BigInt, BigInt, BigInt) {
        if b == big(0) {
            return (a, big(1), big(0));
        }
        let (d1, x1, y1) = RSA::euclides_extendido(b.clone(), a.clone() % b.clone());
        let d = d1.clone();
        let x = y1.clone();
        let y = x1.clone() - (a.clone()/b.clone()) * y1.clone();
        return (d,x,y);
    }

    // implementacion de Miller-Rabin
    // n es el numero que queremos saber si es primo
    // k es el numero de veces que se ejecutara la prueba
    // devuelve verdadero si un entero positivo es primo
    fn es_primo(n:BigUint, k:u64) -> bool{
        // si es 2 o 3, es primo
        if (n == ubig(2)) || (n == ubig(3)) {
            return true;
        }
        // si es par, no es primo 
        if n.clone() & ubig(1) == ubig(0){
            return false;
        }
        // descomponemos a n en la forma (2^r)*d + 1
        let mut r = 0;
        let mut d = n.clone() - ubig(1);
        //mientras d sea par
        while d.clone() & ubig(1) == ubig(0) {
            r += 1;
            d /= ubig(2);
        }
        // hace la prueba k veces
        let mut rng = rand::thread_rng();

    
        fn probar_compuesto(a: BigUint, d:BigUint, n: BigUint, r: u32) -> bool {
            if a.modpow(&d, &n) == ubig(1) {
                return false;
            }
            for j in 0..r {
                if a.modpow(&((2_u64.pow(j)) * d.clone()), &n) == n.clone() - ubig(1){
                    return false;
                }
            }
            return true;
        }

        for _ in 0..k {
            let a = rng.gen_biguint_range(&ubig(2), &n);
            if probar_compuesto(a.clone(),d.clone(),n.clone(),r.clone()) {
                return false;
            }
        }
        //si la prueba no detecta que sea compuesto k veces devuelve verdadero
        return true;
    }

    // genera un numero aleatorio que ademas es impar y de NUM_BITS bits
    fn generar_posible_primo() -> BigUint {
        // generamos un numero aleatorio de num_bits bits
        let mut rng = rand::thread_rng();
        // forzamos a que sea impar y que tenga al menos el numero de digitos especificado
        let mut n = rng.gen_biguint(NUM_BITS);
        n |= (ubig(1) << NUM_BITS - 1) | ubig(1);
        return n;
    }

    //genera un primo aleatorio de NUM_BITS bits
    pub fn generar_primo() -> BigUint{
        let mut p = RSA::generar_posible_primo();
        while !RSA::es_primo(p.clone(), N_PRUEBAS) {
            p = RSA::generar_posible_primo();
        }
        return p;
    }

    //regresa e, un numero menor que phi y coprimo con phi
    // d es su inverso modulo phi
    fn generar_ed(phi:BigUint) -> (BigUint, BigUint){
        let mut e = RSA::generar_posible_primo();
        let (mut g,mut d,_) = RSA::euclides_extendido(e.to_bigint().unwrap(),phi.to_bigint().unwrap());
        while g.clone() != big(1) || e.clone() > phi.clone() {
            e = RSA::generar_posible_primo();
            let (gp,dp,_) = RSA::euclides_extendido(e.to_bigint().unwrap(),phi.to_bigint().unwrap());
            // rust no permite desempacar tuplas usando variables existentes
            g = gp;
            d = dp;
        }
        let u_d: BigUint;
        if d < big(0) {
            u_d = (d % phi.to_bigint().unwrap() + phi.to_bigint().unwrap()).to_biguint().unwrap();
        }
        else {
            u_d = d.to_biguint().unwrap() % phi.clone();
        }
        return (e,u_d);
    }

    // ------------------------ IMPLEMENTACIONES NUEVAS ----------------------------
    
    /// Genera los bigUInts básicos: 0, 1 y 2.
    pub fn gen_biguints_basicos() -> (BigUint, BigUint, BigUint) {
        (ubig(0), ubig(1), ubig(2))
    }

    /// Implementación de la exponenciación modular.
    /// Toma como argumentos la base, el exponente y el módulo.
    pub fn exp_mod(base: &BigUint, exp: &BigUint, md: &BigUint) -> BigUint {
        let mut res = ubig(1);
        let (cero, uno, _) = RSA::gen_biguints_basicos();
        let (mut base, mut exponente) = (base.clone(), exp.clone());

        while exponente > cero {
            if exponente.clone() & uno.clone() > cero {
                res = (res * base.clone()) % md;
            }

            exponente >>= 1;

            base = (base.clone() * base.clone()) % md;
        }

        res 
    }

    /// Función para encriptar un mensaje dado.
    /// Devuelve un vector de bytes.
    #[allow(dead_code)]
    pub fn encriptar(&self,_mensaje:&str) -> Vec<u8> {
        /*
        if !_mensaje.is_ascii() {
            return Err("Error. Por favor elimine los carácteres que no estén en ASCII.")
        } else {
            let m = BigUint::from_bytes_be(_mensaje.as_bytes());
            let c = RSA::exp_mod(&m, &self.e, &self.n);

            let res = BigUint::to_bytes_be(&c); // Orden de bytes: Big Endian.
            
            res
        }
        */
        let m = BigUint::from_bytes_be(_mensaje.as_bytes());
        let c = RSA::exp_mod(&m, &self.e, &self.n);

        let res = BigUint::to_bytes_be(&c); // Orden de bytes: Big Endian.
            
        res
    }

    /// Función para desencriptar un mensaje
    /// previamente encriptado por el algoritmo.
    /// Devuelve una cadena con el mensaje original.
    #[allow(dead_code)]
    //pub fn desencriptar(&self, _byte_array: &[u8]) -> &str {
    pub fn desencriptar(&self, _byte_array: &[u8]) -> String {
        let c = BigUint::from_bytes_be(_byte_array);

        let m = RSA::exp_mod(&c, &self.e, &self.n).to_bytes_be();

        let res = std::str::from_utf8(&m).unwrap().to_string();
        
        res
    }

    /// Función para desencriptar dadas las claves.
    #[allow(dead_code)]
    //pub fn desencriptar_con_clave(_mensaje:&str, _e:BigUint, _n:BigUint) -> &str {
    pub fn desencriptar_con_clave(_mensaje:&str, _e:BigUint, _n:BigUint) -> String {
        let c = BigUint::from_bytes_be(_mensaje.as_bytes());

        let m = RSA::exp_mod(&c, &_e, &_n).to_bytes_be();

        let res = std::str::from_utf8(&m).unwrap().to_string();
        
        res
    }
    /*
    /// Setter para la clave pública.
    #[allow(dead_code)]
    pub fn set_e(&self, _e:BigUint) -> ! {
        self.e = _e;
    }

    /// Setter para la clave privada.
    #[allow(dead_code)]
    pub fn set_n(&self, _n:BigUint) -> ! {
        self.n = _n;
    }
    */
    // ----------------------- FIN IMPLEMENTACIONES NUEVAS --------------------------

    pub fn get_e(&self) -> BigUint {
        self.e.clone()
    }

    pub fn get_n(&self) -> BigUint {
        self.n.clone()
    }
}
 

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    const N_PRUEBAS : u64 =  300; //número de pruebas para es_primo()

    #[test]
    fn test_euclides_ext_1(){
        let result = RSA::euclides_extendido(big(14), big(4));
        assert_eq!(result.0, big(2));
        assert_eq!(result.1, big(1));
        assert_eq!(result.2, big(-3));
    }

    #[test]
    fn test_euclides_ext_2(){
        let result = RSA::euclides_extendido(big(4), big(123));
        assert_eq!(result.0, big(1));
        assert_eq!(result.1, big(31));
        assert_eq!(result.2, big(-1));
    }

    #[test]
    fn test_es_primo_no_primo(){
        let mut rng = rand::thread_rng();
        let comp:u64 = (rng.gen::<u32>() as u64) * 2;
        assert!(!(RSA::es_primo(ubig(comp as u64), N_PRUEBAS)));
    }

    #[test]
    fn test_es_primo_primo(){
        assert!(RSA::es_primo(ubig(13), N_PRUEBAS));
        assert!(RSA::es_primo(ubig(3613), N_PRUEBAS));
        assert!(RSA::es_primo(ubig(10007), N_PRUEBAS));
        assert!(RSA::es_primo(ubig(100000015333), N_PRUEBAS));
    }

    #[test]
    fn test_generar_posible_primo_es_par(){
        let n = RSA::generar_posible_primo();
        assert_eq!(n&ubig(1), ubig(1));
    }

    #[test]
    fn test_generar_primo() {
        for _ in 0..3 {
            let n =  RSA::generar_primo();
            assert!(RSA::es_primo(n, N_PRUEBAS));
        }
    }

    #[test]
    fn test_generar_ed_e_menor_phi(){
        for _ in 0..10 {
            let phi = "77514839631394632856157018692053563208632353428439537437820".parse::<BigUint>().unwrap();
            let (e,d) = RSA::generar_ed(phi.clone());
            println!("este es phi: {}",phi.to_bigint().unwrap());
            println!("este es e: {}",e.to_bigint().unwrap());
            println!("este es d: {}",d);
            let (g, _, _) = RSA::euclides_extendido(e.to_bigint().unwrap(), phi.to_bigint().unwrap());
            println!("este es g: {}", g);
            assert!(e.clone()<phi.clone());
            assert!(g == big(1));
            assert!((e*d)%phi == ubig(1));
        }
    }


}
