# hello_egui

Esercizio per imparare [egui](https://github.com/emilk/egui): una semplice ToDo list desktop scritta in Rust con `eframe` 0.37.

## Avvio

```sh
cargo run
```

## Funzionalità

- Creazione di un ToDo con nome (obbligatorio) e descrizione (opzionale).
- Checkbox per segnare un ToDo come completato (il titolo viene barrato).
- Pulsante **Delete** per rimuovere un ToDo.
- Pulsante **Save** per salvare la lista su `data.csv`.
- All'avvio la lista viene caricata da `data.csv`, se il file esiste.

## Struttura

Tutto il codice è in `src/main.rs`:

- `ToDo`: un elemento della lista. Il metodo `ui` disegna checkbox, titolo e descrizione.
- `ToDoApp`: lo stato dell'applicazione (la lista e i campi del form). Implementa `eframe::App`.
  - `create_form`: form di creazione di un nuovo ToDo.
  - `load_data` / `save_data`: lettura e scrittura di `data.csv`.

## Come funziona egui

egui è una libreria **immediate mode**: a ogni frame il metodo `ui` di `eframe::App` ridisegna l'intera interfaccia a partire dallo stato corrente. Non ci sono widget persistenti né callback: un widget restituisce una `Response`, e si controlla subito se è stato cliccato:

```rust
if ui.button("Delete").clicked() {
    to_remove = Some(i);
}
```

Lo stato vive solo nella struct dell'applicazione (`ToDoApp`), che egui legge e modifica tramite riferimenti `&mut`.

## Formato del file

`data.csv` contiene un ToDo per riga, con i campi separati da `;`:

```
nome;descrizione;completato
```

Il formato è volutamente minimale: nomi o descrizioni che contengono `;` o a capo non vengono gestiti.
