# Materials

materials - is a library of material data.<br>
1 material = 1 struct

Macros are used to create structures.<br>
Materials support localization.

trait | Rock |
| -- | -- |
*Material | ✔ |
🗻Rock | ✔ |
🗻Sedimentary | ✔ |
🗻Igneous | ✔ |
🗻Metamorphic | ✔ |
⚖Density | ✔ |
🌡Melting | ✔ |

`* - has localization`

## Versions

Information on how the library will be updated:

1. X.y.z (X)
    1. X is always incremented. (0 is a proof-of-concept/test version)
    1. Updating X means that the library has changed significantly and the project may need to be rewritten to match the library.
    1. When updating X (for example, from 1 to 2), y and z will continue to be released for versions of X. Material data will remain current, but functionality will be limited to version X.
    1. X is updated when functionality/data is removed (changes incompatible with the previous version of X).
    1. X is an organizational concept; at best, X is not updated.
    1. Be careful when updating a package to the latest X.
1. x.Y.z (Y)
    1. Y is always incremented.
    1. When updating X, Y is not reset to 0.
    1. Y is updated when new functionality is added (not removed). If any functionality is removed, it will first be marked as deprecated. 1. Y is like a new column in a table or a separate table with data. <br>(*libraries are not tables*)
    1. Be calm when updating a package to the latest Y.
    1. Use `materials = "~x.y.z"` to update only Y and Z and ensure that the update is safe.
1. x.y.Z (Z)
    1. Z always increments.
    1. When updating X or Y, Z is not reset to 0.
    1. Z is updated when a material is added or when material data changes, such as changing a name or melting point, etc.
    1. Z is like a new row in a table. <br>(*libraries are not tables*)
    1. Be calm when updating a package to the latest Z.
    1. Use `materials = "^x.y.z"` to update only Z and ensure that the update is safe.

* The main version with the number zero (0.y.z) is intended for initial development. Any changes or deletions may occur at any time.

<hr>

Информация о том как будет обновляться библиотека:

1. X.y.z (X) 
    1. X всегда увеличивается. (0 - это версия-концепция/тест)
    1. Обновление X означает, что библиотека существенно изменилась и может потребоваться переписывание проекта для соответствия библиотеке. 
    1. При обновлении X (к примеру с 1 до 2) продолжится выход y и z для версий X, данные о материалах будут актуальны, но функционал будет ограничен версией X.
    1. X обновляется при удалении функционала/данных.<br>(изменения несовместимые с прошлой версией X)
    1. X - это идея организации данных, в лучшем случае X не обновляется.
    1. Быть внимательным при обновлении пакета до последней X.
1. x.Y.z (Y)
    1. Y всегда увеличивается.
    1. При обновлении X, Y не сбрасывается до 0.
    1. Y обновляется при добавлении (не удалении) нового функционала, если какой то функционал будет удаляться - то сначала оно будет помечено устаревшим.
    1. Y - это как новая колонка в таблице или отдельная таблица с данными. <br>(*библиотека не является таблицами*)
    1. Быть спокойным при обновлении пакета до последней Y.
    1. Используйте `materials = "~x.y.z"`, чтобы обновить только Y и Z и убедиться, что обновление безопасно. 
1. x.y.Z (Z) 
    1. Z всегда увеличивается.
    1. При обновлении X или Y, Z не сбрасывается до 0.
    1. Z обновляется при добавлении материала или при изменении данных о материале, к примеру исправление названия или температуры плавления и т.д.
    1. Z - это как новая строчка в таблице. <br>(*библиотека не является таблицами*)
    1. Быть спокойным при обновлении пакета до последней Z.
    1. Используйте `materials = "^x.y.z"`, чтобы обновить только Z и убедиться, что обновление безопасно.

* Основная версия с нулевым номером (0.y.z) предназначена для начальной разработки. В любой момент что-то может измениться (удалиться).
