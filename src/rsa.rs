//RECORDATORIO: quitar el guión bajo antes de los parámetros después
// de implementar la función y quitar #[allow(dead_code)] cuando de que
// la función sea usada en otro lado.

const MESSAGE_MAX_LENGTH: usize = 257;

pub struct RSA{
    n: u64,
    e: u64,
    _d: u64,
}

impl RSA {
    #[allow(dead_code)]
    pub fn new() -> RSA{
        let p = RSA::generar_primo();
        let q = RSA::generar_primo();
        let n1 = p*q;
        let phi = (p-1)*(q-1);
        let (e1, d1) = RSA::generar_ed(phi);
            
        RSA {
            n: n1,
            e: e1,
            _d: d1
        }
    }   
        
    #[allow(dead_code)]
    pub fn euclides_extendido(_a:i64, _b:i64) -> (i64, i64, i64){
        (0,0,0)
    }

    #[allow(dead_code)]
    pub fn es_primo(_n:u64, _k:u64) -> bool{
        false
    }

    #[allow(dead_code)]
    fn generar_posible_primo() -> u64{
        0
    }

    pub fn generar_primo() -> u64{
        1
    }

    fn generar_ed(_phi:u64) -> (u64, u64){
        (0,0)
    }

    pub fn desencriptar_con_clave(_mensaje:&str, _e:u64, _n:u64) -> &str{
        ""
    }

    #[allow(dead_code)]
    pub fn encriptar(&self,_mensaje:&str) -> [u8;MESSAGE_MAX_LENGTH]{
        [0;MESSAGE_MAX_LENGTH]
    }

    #[allow(dead_code)]
    pub fn desencriptar(&self, _byte_array:[u8;MESSAGE_MAX_LENGTH]) -> &str{
        ""
    }

    pub fn get_e(&self) -> u64{
        self.e
    }

    pub fn get_n(&self) -> u64{
        self.n
    }
}
 

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    const N_PRUEBAS : u64 =  300; //número de pruebas para es_primo()

    #[test]
    #[should_panic]
    fn test_panic_euclides_ext(){
        RSA::euclides_extendido(2, -10);
    }

    #[test]
    fn test_euclides_ext_1(){
        let result = RSA::euclides_extendido(14, 4);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 1);
        assert_eq!(result.2, -3);
    }

    #[test]
    fn test_euclides_ext_2(){
        let result = RSA::euclides_extendido(4, 123);
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 31);
        assert_eq!(result.2, -1);
    }

    #[test]
    fn test_es_primo_no_primo(){
        let mut rng = rand::thread_rng();
        let comp:u64 = (rng.gen::<u32>() as u64) * 2;
        assert!(!(RSA::es_primo(comp as u64, N_PRUEBAS)));
    }

    #[test]
    fn test_es_primo_primo(){
        assert!(RSA::es_primo(13, N_PRUEBAS));
        assert!(RSA::es_primo(2315, N_PRUEBAS));
        assert!(RSA::es_primo(10007, N_PRUEBAS));
    }

    #[test]
    fn test_generar_posible_primo_es_par(){
        let n = RSA::generar_posible_primo();
        assert_eq!((n&1), 1);
    }

    #[test]
    fn test_generar_ed_e_menor_phi(){
        let phi = 104728; //phi(104729)=104728
        let n = RSA::generar_ed(phi);
        assert!(n.0<phi);
    }

}
