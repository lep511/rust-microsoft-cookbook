Athena soporta la mayoría de los tipos de datos de Hive, ya que está basado en Presto/Trino y tiene alta compatibilidad con el ecosistema Hadoop. Aquí están los tipos de datos Hive que Athena soporta:

**Tipos primitivos:**
- BOOLEAN
- TINYINT
- SMALLINT
- INT/INTEGER
- BIGINT
- FLOAT
- DOUBLE
- DECIMAL
- STRING
- VARCHAR
- CHAR
- BINARY
- DATE
- TIMESTAMP
- ARRAY
- MAP
- STRUCT

**Tipos complejos:**
- ARRAY<tipo>: Colecciones ordenadas de elementos del mismo tipo
- MAP<clave_tipo, valor_tipo>: Colecciones de pares clave-valor
- STRUCT<nombre1:tipo1, nombre2:tipo2, ...>: Colecciones de campos nombrados

**Notas importantes:**
- Para el tipo DECIMAL, Athena soporta DECIMAL(precisión, escala)
- A diferencia de Hive tradicional, Athena no soporta completamente los tipos UNION
- Algunos tipos como INTERVAL no son totalmente compatibles

Athena también soporta varias funciones de conversión de tipos para realizar castings entre diferentes tipos de datos cuando sea necesario.