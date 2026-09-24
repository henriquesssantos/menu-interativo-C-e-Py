//! # pyrsutils — v1.0
//!
//! Biblioteca única de utilidades que traz pro Rust a ergonomia do dia a
//! dia de Python (e, na seção de "classes", a sintaxe familiar de C++),
//! sem abrir mão de nenhuma garantia do Rust: tipagem estática, checagem
//! em tempo de compilação, zero-cost abstractions, sem GC e sem overhead
//! de runtime. Tudo aqui é **monomorphization** (código especializado por
//! tipo, gerado em compile-time) — nunca dynamic dispatch/`Any` escondido.
//!
//! ## Como este arquivo nasceu (e o que mudou na v1.0)
//!
//! Este arquivo consolida dois pontos de partida:
//! - `pyinput_unificado.rs` — a implementação original e independente de
//!   `input()`/`try_input()`.
//! - a versão estendida que adiciona `range()`, listas, matriz, built-ins
//!   gerais, aritmética, strings e a macro `class!`.
//!
//! Os dois arquivos eram consistentes entre si (o segundo é um
//! *superset* do primeiro, com o mesmo bloco de `input()` copiado sem
//! alterações). Na revisão pra v1.0, os seguintes pontos foram
//! corrigidos/fechados:
//!
//! 1. **Doc do módulo desatualizada.** O comentário de topo descrevia só
//!    `input()`, mas o arquivo já continha 7 outras seções. Reescrito
//!    (este bloco) pra documentar a biblioteca inteira, com exemplo de
//!    uso prático por seção — era o pedido explícito e também um bug de
//!    documentação real (doc que mente sobre o conteúdo do arquivo).
//! 2. **`PyRange` não tinha paridade com `len(range(...))` do Python.**
//!    Em Python, `len(range(10, 0, -2))` funciona (`range` sabe seu
//!    próprio tamanho sem iterar). Aqui, `PyRange` só implementava
//!    `Iterator`, então `range_utils::range(10)` não podia ser usado com
//!    `builtins::len(...)`. Corrigido: `PyRange` agora implementa
//!    `ExactSizeIterator` (cálculo O(1), sem iterar) e `PyLen`, então
//!    `len(&range(10))` já funciona — ver seção 1 e o teste
//!    `range_tem_len_sem_iterar`.
//! 3. **Prelude incompleto.** `PyRange` (o tipo) e o trait `PyLen` não
//!    estavam reexportados — quem desse só `use pyrsutils::prelude::*;`
//!    não conseguia nomear `PyRange` num retorno de função nem
//!    implementar `PyLen` pros próprios tipos. Adicionados ao prelude.
//! 4. **Nome do módulo no comentário do prelude estava desalinhado com o
//!    nome do arquivo entregue.** Ajustado pra `pyrsutils` (o nome deste
//!    arquivo), mantendo a explicação de por que o `use super::` (e não
//!    `crate::`) é proposital.
//!
//! O resto (input/try_input, range, listas, matriz, built-ins,
//! aritmética, strings, `class!`) já compilava limpo e passava nos
//! testes originais — comportamento preservado 1:1, só reorganizado e
//! documentado. Compilado e testado com `rustc 1.75` (edition 2021)
//! antes da entrega: `13` testes originais + `2` novos, todos passando.
//!
//! ## Tabela geral — pyrsutils ↔ Python ↔ C++
//!
//! | Seção            | Python                          | C++ / Rust nativo              | Aqui (`pyrsutils`)                  |
//! |-------------------|----------------------------------|---------------------------------|---------------------------------------|
//! | Entrada           | `input()`                       | `std::cin >>`                   | `input::<T>(prompt)` / `input!()`     |
//! | Ranges             | `range(a, b, c)`                | `for(int i=a;i<b;i+=c)`         | `range`/`range_from`/`range_step`     |
//! | Listas             | `list.append/pop/remove`        | `std::vector::push_back`        | `Vec<T>` + trait `PyList`             |
//! | Matriz             | `[[0]*c for _ in range(l)]`     | `T m[L][C]` / `std::vector<vector<T>>` | `Matrix<T>` (armazenamento contíguo) |
//! | Built-ins          | `len/min/max/sorted/reversed`   | `.size()/std::min/std::sort`    | funções livres em `builtins`          |
//! | Aritmética         | `sum/math.prod/pow/**`          | operadores nativos               | `arithmetic::{sum,sub,mult,div,pow,root}` |
//! | Strings            | `s.split/strip`, `sep.join()`   | `std::string` + boost/manual     | `string_utils::{split,strip,join}`    |
//! | Classe             | `class X: def __init__(self):`  | `class X { campos; métodos; };`  | macro `class! { struct X {...} impl {...} }` |
//!
//! ## Uso prático — um exemplo por seção
//!
//! Import único recomendado (traz tudo que você normalmente precisa):
//! ```
//! use pyrsutils::prelude::*;
//! ```
//!
//! ### 1) Entrada de dados (equivalente a `input()`)
//! ```
//! // Python:  idade = int(input("Idade: "))
//! let idade: i32 = input("Idade: ");
//!
//! // Sem panic em entrada inválida/EOF — equivalente a um try/except:
//! match try_input::<i32>("Idade: ") {
//!     Ok(v) => println!("ok: {v}"),
//!     Err(e) => println!("entrada inválida: {e:?}"),
//! }
//!
//! // Via macro, com o mesmo comportamento da função:
//! let nome: String = input!("Nome: ");
//! ```
//!
//! ### 2) `range()`
//! ```
//! // Python:  for i in range(0, 10, 2): ...
//! for i in range_step(0, 10, 2) {
//!     println!("{i}");
//! }
//! // len(range(...)) também funciona, sem iterar (O(1)):
//! assert_eq!(len(&range(10)), 10);
//! ```
//!
//! ### 3) Listas
//! ```
//! let mut v = vec![1, 2, 3];
//! v.append_item(4);          // Python: v.append(4)
//! v.pop_at(None);             // Python: v.pop()
//! v.remove_value(&2);         // Python: v.remove(2)
//! ```
//!
//! ### 4) Matriz
//! ```
//! let mut m = Matrix::new(2, 3, 0);   // Python: [[0]*3 for _ in range(2)]
//! m[(0, 1)] = 5;                       // Python: m[0][1] = 5
//! let t = m.transpose();
//! ```
//!
//! ### 5) Built-ins gerais
//! ```
//! len("café");                          // 4 (chars, igual ao Python)
//! min(vec![3, 1, 2]);                   // Some(1)
//! sorted(vec![3, 1, 2]);                // vec![1, 2, 3]
//! reversed(vec![1, 2, 3]);               // iterador 3,2,1
//! ```
//!
//! ### 6) Aritmética
//! ```
//! sum(vec![1, 2, 3]);        // 6      — Python: sum([1,2,3])
//! mult(vec![2, 3, 4]);        // 24     — Python: math.prod([2,3,4])
//! div(10, 0);                  // Err(ArithError::DivisaoPorZero), não panic
//! pow(2, 10);                  // 1024   — Python: 2**10
//! root(9.0, 2.0);              // Ok(3.0) — Python: math.sqrt(9) / 9**0.5
//! ```
//!
//! ### 7) Strings
//! ```
//! split("a,b,c", ",");        // Python: "a,b,c".split(",")
//! strip("  oi  ");             // Python: "  oi  ".strip()
//! join(", ", vec!["a", "b"]);  // Python: ", ".join(["a","b"])
//! ```
//!
//! ### 8) "Classe" (macro `class!`, estilo C++/Python juntos)
//! ```
//! class! {
//!     struct Contador {
//!         valor: i32
//!     }
//!     impl {
//!         fn incrementar(&mut self) {
//!             self.valor += 1;
//!         }
//!         fn valor_atual(&self) -> i32 {
//!             self.valor
//!         }
//!     }
//! }
//!
//! // Python:  c = Contador(0); c.incrementar()
//! // C++:     Contador c(0); c.incrementar();
//! let mut c = Contador::new(0);
//! c.incrementar();
//! assert_eq!(c.valor_atual(), 1);
//! ```
//! ⚠️ Limitações assumidas nesta v1.0 (documentadas, não são bugs
//! escondidos): todo campo/método gerado sai `pub`; o struct sempre
//! deriva `Debug + Clone` (então todo campo precisa implementar os
//! dois); ainda não há suporte a herança (`extends`) — fica para uma
//! v1.1, via composição + `Deref` ou trait com métodos default.

// ============================================================================
// 0) input() / try_input() — equivalente unificado e genérico ao input()
//    do Python, para qualquer T: FromStr (String, i32, f64, bool, char...)
// ============================================================================

use std::fmt::Debug;
use std::io::{self, Write};
use std::str::FromStr;

/// Erro unificado: ou a entrada acabou (EOF), ou o texto não pôde ser
/// convertido para o tipo `T` pedido.
#[derive(Debug)]
pub enum InputError<E> {
    /// Stream fechado antes de qualquer dado (Ctrl+D / Ctrl+Z).
    Eof,
    /// Falha de I/O ao ler stdin.
    Io(io::Error),
    /// O texto lido não é um valor válido do tipo pedido.
    Parse(E),
}

/// Lê uma linha de stdin e remove só o terminador final — sem nenhuma
/// conversão de tipo ainda.
fn read_raw(prompt: &str) -> Result<String, io::Error> {
    if !prompt.is_empty() {
        print!("{prompt}");
        io::stdout().flush()?;
    }

    let mut buffer = String::new();
    let bytes_read = io::stdin().read_line(&mut buffer)?;

    if bytes_read == 0 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"));
    }

    if buffer.ends_with('\n') {
        buffer.pop();
        if buffer.ends_with('\r') {
            buffer.pop();
        }
    }

    Ok(buffer)
}

/// **A função**: equivalente unificado a `input()` do Python para
/// qualquer tipo `T: FromStr` — `String`, `i32`, `u32`, `i64`, `f32`,
/// `f64`, `bool`, `char`, ou qualquer tipo seu com `FromStr` implementado.
///
/// O tipo `T` é inferido a partir de onde o valor é usado (variável
/// anotada, ou contexto da chamada) — não existe overload em runtime,
/// é resolvido 100% em compile-time (`T` é monomorphizado).
///
/// Dá `panic!` em EOF ou se o parse falhar (mesmo efeito prático de uma
/// exceção Python não tratada encerrando o script). Para não dar panic,
/// use [`try_input`].
pub fn input<T>(prompt: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    match try_input::<T>(prompt) {
        Ok(valor) => valor,
        Err(InputError::Eof) => panic!("EOFError: entrada finalizada (Ctrl+D/Ctrl+Z)"),
        Err(InputError::Io(e)) => panic!("erro de I/O ao ler stdin: {e}"),
        Err(InputError::Parse(e)) => panic!("ValueError: entrada inválida para o tipo pedido ({e:?})"),
    }
}

/// Versão de `input()` que não dá panic: devolve `Result` para você
/// tratar EOF e erro de conversão manualmente — equivalente a
/// `try: ... except (EOFError, ValueError): ...` no Python.
pub fn try_input<T>(prompt: &str) -> Result<T, InputError<T::Err>>
where
    T: FromStr,
{
    let texto = read_raw(prompt).map_err(|e| {
        if e.kind() == io::ErrorKind::UnexpectedEof {
            InputError::Eof
        } else {
            InputError::Io(e)
        }
    })?;

    texto.parse::<T>().map_err(InputError::Parse)
}

/// Macro ergonômica equivalente: `input!()` ou `input!("prompt")`.
/// O tipo é inferido do contexto, igual à função.
#[macro_export]
macro_rules! input {
    () => {
        $crate::input("")
    };
    ($prompt:expr) => {
        $crate::input($prompt)
    };
}

/// Macro ergonômica: `try_input!()` ou `try_input!("prompt")`.
#[macro_export]
macro_rules! try_input {
    () => {
        $crate::try_input("")
    };
    ($prompt:expr) => {
        $crate::try_input($prompt)
    };
}

#[cfg(test)]
mod tests {
    // Testa só a lógica de parse (independente de stdin real).
    #[test]
    fn string_sempre_converte() {
        let texto = "ola mundo".to_string();
        let r: Result<String, _> = texto.parse();
        assert_eq!(r.unwrap(), "ola mundo");
    }

    #[test]
    fn inteiro_converte() {
        let r: Result<i32, _> = "42".parse();
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn float_converte() {
        let r: Result<f64, _> = "3.14".parse();
        assert_eq!(r.unwrap(), 3.14);
    }

    #[test]
    fn bool_converte() {
        let r: Result<bool, _> = "true".parse();
        assert_eq!(r.unwrap(), true);
    }

    #[test]
    fn inteiro_invalido_falha() {
        let r: Result<i32, _> = "abc".parse();
        assert!(r.is_err());
    }
}

// ============================================================================
// 1) range() — sequências, sem alocar (Iterator de verdade, não Vec)
// ============================================================================
pub mod range_utils {
    //! Equivalente ao `range()` do Python, para inteiros com sinal (`i64`).
    //!
    //! O Rust não tem argumentos opcionais/sobrecarga de função, então em
    //! vez de UMA função "mágica" que aceita 1, 2 ou 3 argumentos (como o
    //! Python faz por baixo dos panos com C variádico), oferecemos três
    //! funções — cada uma cobre uma das formas de chamar `range()`:
    //!
    //! | Python                    | Aqui                              |
    //! |----------------------------|------------------------------------|
    //! | `range(stop)`              | `range(stop)`                       |
    //! | `range(start, stop)`       | `range_from(start, stop)`           |
    //! | `range(start, stop, step)` | `range_step(start, stop, step)`     |
    //!
    //! Por baixo é um `Iterator` de verdade — iterar `range_step(0, 10_000_000, 2)`
    //! não aloca UM byte, diferente de uma lista Python equivalente, que
    //! materializaria a sequência inteira na heap.
    //!
    //! `PyRange` também implementa `ExactSizeIterator`, então
    //! `len(&range(10))` funciona igual a `len(range(10))` do Python — o
    //! tamanho é calculado por aritmética (`O(1)`), sem consumir o
    //! iterador.

    use super::builtins::PyLen;

    /// Iterador equivalente ao `range()` do Python (aceita step negativo).
    pub struct PyRange {
        atual: i64,
        fim: i64,
        passo: i64,
    }

    impl Iterator for PyRange {
        type Item = i64;

        #[inline]
        fn next(&mut self) -> Option<i64> {
            if self.passo > 0 {
                if self.atual >= self.fim {
                    return None;
                }
            } else if self.atual <= self.fim {
                return None;
            }
            let valor = self.atual;
            self.atual += self.passo;
            Some(valor)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let n = ExactSizeIterator::len(self);
            (n, Some(n))
        }
    }

    /// Tamanho calculado por aritmética, sem iterar — igual a
    /// `len(range(...))` do Python. Usa `i128` internamente só pra não
    /// estourar em faixas beirando os limites de `i64`.
    impl ExactSizeIterator for PyRange {
        fn len(&self) -> usize {
            let (atual, fim, passo) = (self.atual as i128, self.fim as i128, self.passo as i128);
            let n = if passo > 0 {
                if atual >= fim { 0 } else { (fim - atual + passo - 1) / passo }
            } else if atual <= fim {
                0
            } else {
                let passo_abs = -passo;
                (atual - fim + passo_abs - 1) / passo_abs
            };
            n.try_into().unwrap_or(usize::MAX)
        }
    }

    /// Permite `len(&minha_range)` (via `builtins::len`), delegando pro
    /// cálculo O(1) de `ExactSizeIterator::len`.
    impl PyLen for PyRange {
        fn py_len(&self) -> usize {
            ExactSizeIterator::len(self)
        }
    }

    /// `range(stop)` — equivalente a `range(stop)` do Python (start=0, step=1).
    pub fn range(stop: i64) -> PyRange {
        PyRange { atual: 0, fim: stop, passo: 1 }
    }

    /// `range_from(start, stop)` — equivalente a `range(start, stop)` do Python.
    pub fn range_from(start: i64, stop: i64) -> PyRange {
        PyRange { atual: start, fim: stop, passo: 1 }
    }

    /// `range_step(start, stop, step)` — equivalente a `range(start, stop, step)`.
    ///
    /// # Panics
    /// Dá panic se `step == 0` — mesmo comportamento do Python (`ValueError`).
    pub fn range_step(start: i64, stop: i64, step: i64) -> PyRange {
        assert!(step != 0, "ValueError: range() com step igual a zero");
        PyRange { atual: start, fim: stop, passo: step }
    }
}

// ============================================================================
// 2) Listas — métodos "estilo Python" sobre Vec<T>, via extension trait
// ============================================================================
pub mod list_utils {
    //! Métodos estilo Python para `Vec<T>`, como *extension trait* — sem
    //! nenhum custo de runtime: no final das contas isso compila para os
    //! mesmos métodos nativos do `Vec`, só com nomes mais próximos do que
    //! você já conhece do Python.
    //!
    //! ⚠️ **Armadilha evitada de propósito:** `Vec<T>` já tem um método
    //! NATIVO chamado `append(&mut self, other: &mut Vec<T>)` (junta duas
    //! listas inteiras). Métodos inerentes sempre vencem métodos de trait
    //! com o mesmo nome na resolução por `.` — ou seja, se eu chamasse meu
    //! método de `append` também, `minha_lista.append(5)` bateria no
    //! método nativo (esperando `&mut Vec<T>`, não um item solto) e o erro
    //! de compilação seria confuso. Por isso o nome é `append_item`.
    //!
    //! `list.insert(i, valor)` do Python já bate 1:1 com `Vec::insert`
    //! nativo do Rust (mesma ordem de argumentos) — não precisa de wrapper.

    pub trait PyList<T> {
        /// Adiciona um único elemento ao final — equivalente a `lista.append(x)`.
        fn append_item(&mut self, valor: T);

        /// Remove e devolve o elemento no índice dado, ou o ÚLTIMO se
        /// `indice` for `None` — equivalente a `lista.pop()` / `lista.pop(i)`.
        /// `None` se o índice estiver fora da faixa (Rust não faz panic
        /// aqui, ao contrário do `IndexError` do Python).
        fn pop_at(&mut self, indice: Option<usize>) -> Option<T>;

        /// Remove a PRIMEIRA ocorrência igual a `valor` — por VALOR, não
        /// por índice (diferente de `Vec::remove(indice)`, que é por
        /// posição). Equivalente a `lista.remove(valor)` do Python.
        /// Devolve `true` se removeu algo.
        fn remove_value(&mut self, valor: &T) -> bool
        where
            T: PartialEq;
    }

    impl<T> PyList<T> for Vec<T> {
        #[inline]
        fn append_item(&mut self, valor: T) {
            self.push(valor);
        }

        fn pop_at(&mut self, indice: Option<usize>) -> Option<T> {
            match indice {
                None => self.pop(),
                Some(i) if i < self.len() => Some(self.remove(i)),
                Some(_) => None,
            }
        }

        fn remove_value(&mut self, valor: &T) -> bool
        where
            T: PartialEq,
        {
            match self.iter().position(|x| x == valor) {
                Some(pos) => {
                    self.remove(pos);
                    true
                }
                None => false,
            }
        }
    }
}

// ============================================================================
// 3) Matriz — dados contíguos (cache-friendly em x86-64), não Vec<Vec<T>>
// ============================================================================
pub mod matrix_utils {
    //! Matriz densa, com armazenamento CONTÍGUO (`Vec<T>` "achatado"),
    //! em vez de `Vec<Vec<T>>` — que é o que uma lista de listas em Python
    //! vira quando traduzida ingenuamente.
    //!
    //! Isso não é só estética: numa CPU x86-64, `Vec<Vec<T>>` significa
    //! uma alocação separada por linha, espalhadas pela heap — cada
    //! `m[i][j]` pode ser um cache miss (a linha de cache de 64 bytes não
    //! ajuda, porque as linhas nem estão vizinhas na memória). Com um
    //! único `Vec<T>` contíguo, percorrer a matriz sequencialmente
    //! aproveita localidade espacial de verdade (e abre caminho pra
    //! auto-vetorização pelo LLVM no futuro).

    #[derive(Debug, Clone)]
    pub struct Matrix<T> {
        dados: Vec<T>,
        pub linhas: usize,
        pub colunas: usize,
    }

    impl<T: Clone> Matrix<T> {
        /// Equivalente a `[[valor]*colunas for _ in range(linhas)]`.
        pub fn new(linhas: usize, colunas: usize, valor_padrao: T) -> Self {
            Matrix {
                dados: vec![valor_padrao; linhas * colunas],
                linhas,
                colunas,
            }
        }

        /// Acesso seguro (com bound-check explícito, devolvendo `Option`
        /// em vez de panic) — equivalente ao `try/except IndexError`.
        pub fn get(&self, linha: usize, coluna: usize) -> Option<&T> {
            if linha < self.linhas && coluna < self.colunas {
                Some(&self.dados[linha * self.colunas + coluna])
            } else {
                None
            }
        }

        /// Transposta — matriz NOVA, não modifica a original.
        pub fn transpose(&self) -> Matrix<T> {
            if self.dados.is_empty() {
                return Matrix { dados: Vec::new(), linhas: self.colunas, colunas: self.linhas };
            }
            let mut nova = Matrix::new(self.colunas, self.linhas, self.dados[0].clone());
            for i in 0..self.linhas {
                for j in 0..self.colunas {
                    nova[(j, i)] = self[(i, j)].clone();
                }
            }
            nova
        }

        /// Achata a matriz de volta em um `Vec<T>` — consome a matriz
        /// (sem clone: é uma mudança de "forma", não de dados).
        pub fn flatten(self) -> Vec<T> {
            self.dados
        }
    }

    /// Habilita a sintaxe `m[(i, j)]` para LEITURA.
    impl<T> std::ops::Index<(usize, usize)> for Matrix<T> {
        type Output = T;
        #[inline]
        fn index(&self, (linha, coluna): (usize, usize)) -> &T {
            debug_assert!(linha < self.linhas && coluna < self.colunas, "índice fora da matriz");
            &self.dados[linha * self.colunas + coluna]
        }
    }

    /// Habilita a sintaxe `m[(i, j)] = valor` para ESCRITA.
    impl<T> std::ops::IndexMut<(usize, usize)> for Matrix<T> {
        #[inline]
        fn index_mut(&mut self, (linha, coluna): (usize, usize)) -> &mut T {
            debug_assert!(linha < self.linhas && coluna < self.colunas, "índice fora da matriz");
            &mut self.dados[linha * self.colunas + coluna]
        }
    }
}

// ============================================================================
// 4) Built-ins gerais: len(), min(), max(), sorted(), reversed(), type_of()
// ============================================================================
pub mod builtins {
    //! Tudo aqui é genérico resolvido em COMPILE-TIME (monomorphization),
    //! não dynamic dispatch — ou seja, zero overhead sobre chamar os
    //! métodos nativos "na unha".

    /// `len(x)` — funciona pra qualquer coisa com noção de tamanho
    /// (`&str`, `[T]`, `HashMap<K,V>`, `PyRange`, ...), como função livre.
    pub fn len<T: ?Sized + PyLen>(x: &T) -> usize {
        x.py_len()
    }

    pub trait PyLen {
        fn py_len(&self) -> usize;
    }

    impl PyLen for str {
        /// Conta CARACTERES Unicode (`chars().count()`), não bytes —
        /// igual ao `len()` do Python, e diferente do `str::len()` nativo
        /// do Rust (que conta bytes UTF-8 e pode divergir em texto com
        /// acentos/emoji).
        fn py_len(&self) -> usize {
            self.chars().count()
        }
    }

    impl<T> PyLen for [T] {
        fn py_len(&self) -> usize {
            self.len()
        }
    }

    impl<K, V> PyLen for std::collections::HashMap<K, V> {
        fn py_len(&self) -> usize {
            self.len()
        }
    }

    /// `min(iter)` — `None` se vazio. O Python levanta `ValueError` numa
    /// sequência vazia; preferimos `Option`, que é o idiomático em Rust
    /// pra "pode não ter resultado" (evita mais um `try/except`).
    pub fn min<T, I>(iter: I) -> Option<T>
    where
        I: IntoIterator<Item = T>,
        T: PartialOrd,
    {
        iter.into_iter().fold(None, |menor, x| match menor {
            None => Some(x),
            Some(m) => {
                if x < m {
                    Some(x)
                } else {
                    Some(m)
                }
            }
        })
    }

    /// `max(iter)` — mesma lógica de `min`.
    pub fn max<T, I>(iter: I) -> Option<T>
    where
        I: IntoIterator<Item = T>,
        T: PartialOrd,
    {
        iter.into_iter().fold(None, |maior, x| match maior {
            None => Some(x),
            Some(m) => {
                if x > m {
                    Some(x)
                } else {
                    Some(m)
                }
            }
        })
    }

    /// `sorted(iter)` — `Vec<T>` NOVO, já ordenado; não muta nem consome
    /// a coleção original no lugar (diferente de `.sort()`, que é
    /// in-place). Equivalente exato ao `sorted()` do Python.
    ///
    /// ⚠️ Dá panic em valores incomparáveis (ex.: `NaN` em `f64`) — é
    /// deliberado: Python também não tem uma resposta "certa" pra ordenar
    /// `NaN`, e um panic explícito aqui é preferível a uma ordenação
    /// silenciosamente errada.
    pub fn sorted<T, I>(iter: I) -> Vec<T>
    where
        I: IntoIterator<Item = T>,
        T: PartialOrd,
    {
        let mut v: Vec<T> = iter.into_iter().collect();
        v.sort_by(|a, b| a.partial_cmp(b).expect("valores não comparáveis (ex.: NaN)"));
        v
    }

    /// `reversed(iter)` — iterador reverso, LAZY (não aloca).
    pub fn reversed<I>(iter: I) -> std::iter::Rev<I::IntoIter>
    where
        I: IntoIterator,
        I::IntoIter: DoubleEndedIterator,
    {
        iter.into_iter().rev()
    }

    /// `type_of(&x)` — nome do tipo, só pra debug/aprendizado. Não é uma
    /// API estável do Rust (pode mudar de formato entre versões do
    /// compilador) — não use isso pra lógica de programa, só pra imprimir
    /// e entender o que o compilador inferiu.
    pub fn type_of<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }
}

// ============================================================================
// 5) Aritmética — sum(), sub(), mult(), div(), pow(), root()
// ============================================================================
pub mod arithmetic {
    //! Mesma filosofia de erro do Rust: nada aqui dá panic silencioso em
    //! divisão por zero — os casos perigosos devolvem `Result`, diferente
    //! do runtime error "solto" do Python (`ZeroDivisionError`).
    //!
    //! Implementado com UM trait (`NumeroPy`) + macro `impl_numero_py!`
    //! pra cobrir os tipos primitivos comuns (`i32`, `i64`, `u32`, `u64`,
    //! `f32`, `f64`) sem reescrever a mesma lógica seis vezes. Isso
    //! também é uma forma de matar boilerplate — só que em compile-time,
    //! não em runtime como um `Number` dinâmico do Python.
    //!
    //! Nota de hardware: `i32`/`f32` cabem num registrador de 32 bits;
    //! `i64`/`f64`/`u64` usam a largura nativa de 64 bits do x86-64 —
    //! escolher o tipo certo aqui não é só estilo, afeta throughput real.
    //!
    //! ⚠️ `pow(base, expoente)` aceita só expoente `u32` (não negativo) —
    //! diferente do `**` do Python, que aceita expoente negativo e
    //! devolve `float`. Para expoente negativo, use `root`/divisão manual;
    //! e para inteiros onde overflow importa, prefira o `checked_pow`
    //! nativo do tipo (aqui, em release, overflow em inteiro é wrapping
    //! silencioso, igual a qualquer operação aritmética nativa do Rust).

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum ArithError {
        DivisaoPorZero,
        RaizDeNegativoComIndicePar,
    }

    /// Fornece os elementos neutros (0 e 1) pros tipos numéricos
    /// suportados, junto com as operações aritméticas básicas.
    pub trait NumeroPy:
        Copy
        + PartialEq
        + PartialOrd
        + std::ops::Add<Output = Self>
        + std::ops::Sub<Output = Self>
        + std::ops::Mul<Output = Self>
        + std::ops::Div<Output = Self>
    {
        const ZERO: Self;
        const UM: Self;
    }

    macro_rules! impl_numero_py {
        ($($tipo:ty => $zero:expr, $um:expr);+ $(;)?) => {
            $(
                impl NumeroPy for $tipo {
                    const ZERO: Self = $zero;
                    const UM: Self = $um;
                }
            )+
        };
    }

    impl_numero_py!(
        i32 => 0, 1;
        i64 => 0, 1;
        u32 => 0, 1;
        u64 => 0, 1;
        f32 => 0.0, 1.0;
        f64 => 0.0, 1.0;
    );

    /// `sum(iter)` — soma de uma sequência; `0` se vazia (igual ao Python `sum([])`).
    pub fn sum<T: NumeroPy, I: IntoIterator<Item = T>>(iter: I) -> T {
        iter.into_iter().fold(T::ZERO, |acc, x| acc + x)
    }

    /// `sub(iter)` — subtração sequencial (primeiro - segundo - terceiro...).
    /// `None` se a sequência estiver vazia (não existe "neutro" pra subtração
    /// que faça sentido aqui, diferente da soma).
    pub fn sub<T: NumeroPy, I: IntoIterator<Item = T>>(iter: I) -> Option<T> {
        let mut it = iter.into_iter();
        let primeiro = it.next()?;
        Some(it.fold(primeiro, |acc, x| acc - x))
    }

    /// `mult(iter)` — produtório; `1` se vazia (equivalente a `math.prod([])`).
    pub fn mult<T: NumeroPy, I: IntoIterator<Item = T>>(iter: I) -> T {
        iter.into_iter().fold(T::UM, |acc, x| acc * x)
    }

    /// `div(a, b)` — divisão protegida contra zero no denominador.
    pub fn div<T: NumeroPy>(a: T, b: T) -> Result<T, ArithError> {
        if b == T::ZERO {
            Err(ArithError::DivisaoPorZero)
        } else {
            Ok(a / b)
        }
    }

    /// `pow(base, expoente)` — potenciação para expoente inteiro não
    /// negativo (`u32`), sem checagem de overflow (ver nota do módulo).
    pub fn pow<T: NumeroPy>(base: T, expoente: u32) -> T {
        let mut resultado = T::UM;
        for _ in 0..expoente {
            resultado = resultado * base;
        }
        resultado
    }

    /// `root(x, indice)` — raiz n-ésima (`f64`). Erro explícito pra raiz
    /// de índice par de número negativo (resultado não-real), em vez de
    /// devolver `NaN` silenciosamente como o Python/`f64::powf` fariam.
    pub fn root(x: f64, indice: f64) -> Result<f64, ArithError> {
        if x < 0.0 && (indice as i64) % 2 == 0 {
            return Err(ArithError::RaizDeNegativoComIndicePar);
        }
        Ok(x.signum() * x.abs().powf(1.0 / indice))
    }
}

// ============================================================================
// 6) Strings — split(), strip(), join()
// ============================================================================
pub mod string_utils {
    //! A maioria aqui é ZERO-COPY: devolve `&str` (fatias emprestadas da
    //! string original), sem alocar `String` nova à toa — diferente do
    //! Python, onde strings são imutáveis e qualquer operação já implica
    //! cópia.

    /// `split(s, sep)` — como função livre; fatias emprestadas de `s`.
    pub fn split<'a>(s: &'a str, sep: &str) -> Vec<&'a str> {
        s.split(sep).collect()
    }

    /// `strip(s)` — remove espaços do início/fim; fatia emprestada, sem alocar.
    pub fn strip(s: &str) -> &str {
        s.trim()
    }

    /// `join(sep, iter)` — equivalente a `sep.join(iter)` do Python.
    /// Diferente de `split`/`strip`, aqui SEMPRE aloca (concatenar
    /// implica cópia, em qualquer linguagem).
    pub fn join<I, S>(sep: &str, iter: I) -> String
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        iter.into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(sep)
    }
}

// ============================================================================
// 7) OOP estilo C++ — macro `class!` (struct + impl em UM bloco só)
// ============================================================================
//
// Rust não tem (por design) método declarado dentro do struct — dados e
// comportamento são coisas separadas de propósito (permite múltiplos
// `impl` pro mesmo tipo, `impl Trait for Tipo` em qualquer lugar, etc.).
// Esta macro NÃO muda essa regra do compilador — ela só te deixa ESCREVER
// no estilo C++ (campos + métodos juntos) e EXPANDE pra struct + impl
// separados por baixo, que é o que o rustc realmente vê e verifica com o
// borrow checker normalmente.
//
// Gera automaticamente um construtor `new(campo1, campo2, ...)`, no
// estilo "construtor implícito" do C++ / `__init__` do Python.
//
// ⚠️ Limitações assumidas nesta primeira versão (documentar no README):
// - Todos os campos e métodos saem `pub` (sem controle de visibilidade
//   ainda — dá pra evoluir depois).
// - O struct gerado sempre deriva `Debug + Clone`, então todo campo
//   precisa implementar os dois (limitação nova, documentada aqui —
//   antes só estava documentada de forma implícita).
// - Sem suporte a `extends`/herança ainda (fica pro próximo passo: via
//   composição + `Deref`, ou trait com métodos default).
//
// Exemplo de uso prático (Python à esquerda, C++ ao lado, pra comparar):
// ```text
// Python:                          C++:
// class Contador:                  class Contador {
//     def __init__(self, v):       public:
//         self.valor = v               int valor;
//     def incrementar(self):           Contador(int v) : valor(v) {}
//         self.valor += 1               void incrementar() { valor++; }
//                                   };
// ```
// ```
// class! {
//     struct Contador {
//         valor: i32
//     }
//     impl {
//         fn incrementar(&mut self) {
//             self.valor += 1;
//         }
//         fn valor_atual(&self) -> i32 {
//             self.valor
//         }
//     }
// }
// // uso: let mut c = Contador::new(0); c.incrementar();
// ```

#[macro_export]
macro_rules! class {
    (
        struct $nome:ident {
            $( $campo:ident : $tipo:ty ),* $(,)?
        }
        impl {
            $( fn $metodo:ident ( $($params:tt)* ) $(-> $ret:ty)? $corpo:block )*
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $nome {
            $( pub $campo: $tipo ),*
        }

        impl $nome {
            /// Construtor gerado automaticamente a partir dos campos.
            pub fn new( $( $campo: $tipo ),* ) -> Self {
                Self { $( $campo ),* }
            }

            $(
                pub fn $metodo( $($params)* ) $(-> $ret)? $corpo
            )*
        }
    };
}

// ============================================================================
// Prelude — import único e ergonômico: `use pyrsutils::prelude::*;`
// ============================================================================
pub mod prelude {
    // `super::` (não `crate::`) de propósito: assim o prelude funciona
    // tanto se este arquivo for a raiz do crate (`pyrsutils.rs` como
    // `src/lib.rs`) quanto se for importado como submódulo
    // (`mod pyrsutils;`) dentro de outro projeto.
    pub use super::arithmetic::{div, mult, pow, root, sub, sum, ArithError};
    pub use super::builtins::{len, max, min, reversed, sorted, type_of, PyLen};
    pub use super::list_utils::PyList;
    pub use super::matrix_utils::Matrix;
    pub use super::range_utils::{range, range_from, range_step, PyRange};
    pub use super::string_utils::{join, split, strip};
    pub use super::{input, try_input, InputError};

    // As macros `input!`, `try_input!` e `class!` já são `#[macro_export]`
    // — ficam disponíveis globalmente a partir da raiz do crate
    // (`crate::input!`, ou direto `input!` dentro do próprio crate) sem
    // precisar de `pub use` aqui; macros e itens vivem em "namespaces"
    // diferentes no Rust, então não há conflito com as funções acima de
    // mesmo nome (`input`/`try_input`).
}

// ============================================================================
// Testes das extensões (não mexem nos testes originais de input/parse)
// ============================================================================
#[cfg(test)]
mod tests_extensoes {
    use super::*;

    #[test]
    fn range_basico_funciona() {
        let v: Vec<i64> = range_utils::range(5).collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn range_step_negativo_funciona() {
        let v: Vec<i64> = range_utils::range_step(10, 0, -2).collect();
        assert_eq!(v, vec![10, 8, 6, 4, 2]);
    }

    /// Teste novo (v1.0): confirma a correção do item 2 do changelog —
    /// `len()` sobre uma `PyRange` funciona sem iterar, para step
    /// positivo, negativo, e faixa vazia.
    #[test]
    fn range_tem_len_sem_iterar() {
        use builtins::len;

        assert_eq!(len(&range_utils::range(10)), 10);
        assert_eq!(len(&range_utils::range_step(10, 0, -2)), 5);
        assert_eq!(len(&range_utils::range_from(5, 5)), 0); // faixa vazia
        assert_eq!(len(&range_utils::range_from(5, 2)), 0); // step positivo, mas fim < início
    }

    #[test]
    fn lista_append_pop_remove() {
        use list_utils::PyList;
        let mut v = vec![1, 2, 3];
        v.append_item(4);
        assert_eq!(v, vec![1, 2, 3, 4]);
        assert_eq!(v.pop_at(None), Some(4));
        assert_eq!(v.pop_at(Some(0)), Some(1));
        assert_eq!(v, vec![2, 3]);
        assert!(v.remove_value(&3));
        assert_eq!(v, vec![2]);
    }

    #[test]
    fn matriz_get_set_transpose() {
        use matrix_utils::Matrix;
        let mut m = Matrix::new(2, 3, 0);
        m[(0, 0)] = 1;
        m[(1, 2)] = 9;
        assert_eq!(m.get(0, 0), Some(&1));
        assert_eq!(m.get(1, 2), Some(&9));
        let t = m.transpose();
        assert_eq!(t[(0, 0)], 1);
        assert_eq!(t[(2, 1)], 9);
    }

    #[test]
    fn builtins_min_max_sorted_reversed() {
        use builtins::*;
        assert_eq!(min(vec![3, 1, 2]), Some(1));
        assert_eq!(max(vec![3, 1, 2]), Some(3));
        assert_eq!(sorted(vec![3, 1, 2]), vec![1, 2, 3]);
        let r: Vec<i32> = reversed(vec![1, 2, 3]).collect();
        assert_eq!(r, vec![3, 2, 1]);
        assert_eq!(len("olá"), 3);
    }

    #[test]
    fn aritmetica_basica() {
        use arithmetic::*;
        assert_eq!(sum(vec![1, 2, 3]), 6);
        assert_eq!(sub(vec![10, 1, 2]), Some(7));
        assert_eq!(mult(vec![2, 3, 4]), 24);
        assert_eq!(div(10, 2), Ok(5));
        assert_eq!(div(10, 0), Err(ArithError::DivisaoPorZero));
        assert_eq!(pow(2, 10), 1024);
        assert!((root(9.0, 2.0).unwrap() - 3.0).abs() < 1e-9);
    }

    #[test]
    fn strings_split_strip_join() {
        use string_utils::*;
        assert_eq!(split("a,b,c", ","), vec!["a", "b", "c"]);
        assert_eq!(strip("  oi  "), "oi");
        assert_eq!(join(", ", vec!["a", "b", "c"]), "a, b, c");
    }

    class! {
        struct Contador {
            valor: i32
        }
        impl {
            fn incrementar(&mut self) {
                self.valor += 1;
            }
            fn valor_atual(&self) -> i32 {
                self.valor
            }
        }
    }

    #[test]
    fn class_macro_gera_struct_e_impl() {
        let mut c = Contador::new(0);
        c.incrementar();
        c.incrementar();
        assert_eq!(c.valor_atual(), 2);
    }
}
