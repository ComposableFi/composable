# Overview

This RFC proposes specific guidelines for project item design and communication. 

These guidelines ensure all development teams are aligned, informed, settle the right expectations to each other and work in nice synchrony to deliver company goals.

## Assumptions and expectations

No specific hierarchy items, tags or team organizations are expected.

This RFC assumes there could be out-of-items or out-of-process work. 
So it is considered optional and decided on per team and work at hand.

Expectancy that all cross-team collaboration will adhere to the constraints outlined in this guide. 

In a team, collaboration would adhere to this guide too, until full consensus to do othe flow. Any team member may reject work that is not aligned with the flow outlined here.

RFC does not specifies what happens in some edge cases, example in case of vacations, assuming commons sence is used in these case. 

This RFC assume that we aggreed that `ClickUp`(`CU`) was agreed be `source of truth` of company developmentes processes.  Specifically all work items must entered in CU, with comment outlined before.

Each company member is at least of [Novice Member in CU usage](https://univerasity.clickup.com/page/course-catalog#level_novice,role_member).

## Problem

Currently, development produces wasted output, misses people's expectations on communication channels, request reaction timelines are not clear, and  expected output of one team is not aligned with consumer team. 

We have tooling to make our development more faster and transparenty, but we do not not have agreed on mechanics how to improve our work.

## Solution

We should aligned on flows handling CU items effectiely and efficiently improving on `Problems` outlined above. 

Next is set of guidlines and constrains to operate CU items which are considered good to improvement.

### CU guidlines

`ETA` - is 24 working hours

`Item` - CU item 

#### Commnication

People tagged or mentioned in item or anybody from watching an item are expected to respond within ETA.
GitHub (GH) issues, GH metions or review requests notifications are good help, but not required.
Slack notifications are also good will, but optional.

**Will it work?**

CU is very versatile in delivery, email, mobile app, desktop up, browser, watcher and taging.
Notification table is huge and allows to fine tune every tiny detail of notificaitons. 

That is way more thatn Slack or GH provide. 



#### Depedencies

If there is dependency on exteranal our of companu CU work item work or dependcy, it should be accompanied by in CU referencing work item.
If one team depends on work of other team, CU added as dependency.

#### Item existence

Any work which is not considered to be done or planned should not be in CU. 
Repeating, work which is not considered to be done should not be click up item.

CU creation can be dicussed. If dicussion is long and inteream, task to align/decide should be created.

Task created by single person and not tagging any other people should not exists. Consider that to be `at least 1 reviewer` rule for task.  

Non of tasks could created or watched by single person. If task was created you should make watch or tag with explanation people who should be aware of its existence and progress.

If task was created as result of communiction in GH or Slack, refere GH item or Slack dicussion (or people from diucssion) in CU item.

**Will it work?**

Producer  from Electorns Arts said that in one startip, and it made project managment tool more clean and targeted. 

It is good way to clean up grabage. 

Also it impvoe item description and solvence as at least 2 people are on what this item is about.




#### Item state

If person is not working on CU item during 24 hours (or not going to do so) it changes status to ensure it is NOT  `in progress` item. You should leave comment if you plan to proceed later.
When task is started to work on, it set to equivalent of `progress state`.
If you stop any work on item and not going to do any progress, you unassign task from you. Settle expectations by setting some final end status  or set some intermediate status assigned with assiganation other people or do 3, but remove you from assigned and add comentary why you are doing so.
If task assigned to you and it is not clear why it was happend, you have 24 working hours to ask for clarification.
If any work was considered to be and decided not to be later, it should be set to end state by current assigen or its manager.


#### Subitems

If any item takes more then several days to deliver, split it into subtask with resonable delivery goals.
If task grows large because of unvoered new items, limit scope of this task deliver and outline new tasks which scope remaining work. Explain why this is done.
If work item is assigned to several people and in progress. Specific subitem it outlined assigned to one person.

** Will it work?**

It allwos personal process to fin into global work. Also it is very agile to attune to uncovered detiles.
Smaller task are more predictable, more parallasible.
Too big items.

Also it clarified who does what.

- One startup i worked (Rust).
- Half year.



## Notes

This guidlines compose nicely with per item `definition of done` or/and `acceptance criteria` are outliend in each item.

** Will it work? **

How it will work without?

Having boards wich bring several layers of subtasks into meeting based on `tags` and `labels` well compose with subitems creations guidlines outlined above.

** will it work**?

It allows to fit subtasks into board and ghant chart as soon as these created. 

Only anectodate perosnal evidence. So more tooling appears in that area as time goes.

This guideline also compose nicely with automatically recalculaed `Gantt Chart`.

### Ghant Chart

To undrestand how it composes, need to understand what it is.

Having `inputs`:
1. Items to things done.
2. Items on which items depend on. 
3. Resource units to execute items. Each resource unit has limited single thread time per day. Examle, 2 Rust develope, 1 QA.
4. Estimates on raw time to be spent to each resource. There is no unqalified not resourced time.
4.1 Items parent of several items includes only time which it requires to finalise items.

Running Ghant Chart in it,
1. Will paralelise work as much as possible give dependnecy graph and qualified resources. 
1.1 So it will estimate work with parallizastion time and avoid mistakes on prediciton based only on linear time of full occupation
1.2 Will revela resource bottlenecks
2. It will predict deliver time
2.1 Changes in items and work will changed predicted time
2.2. Real time of deliver can be ajuested based on snapshots of previos predicictions taken over the time.
2.3. And will allow to make feedback look to improve seting up `inputs` better next time.
3. Will give nice and visual picture.

As you can see this approach is more automated and scinetic and more maintainble then drawing some `ghang chart` by hand.

LibreOffice.

It well composes with `Substask` creation process.

**Will it work?**

- team
- estimates


## TODO

Антон Кот, [2022-11-30 9:34 PM]
"Producer from Electorns Arts said that in one startip, and it made project managment tool more clean and targeted. 
It is good way to clean up grabage. 
Also it impvoe item description and solvence as at least 2 people are on what this item is about."

Антон Кот, [2022-11-30 9:36 PM]
"It allwos personal process to fin into global work. Also it is very agile to attune to uncovered detiles.Smaller task are more predictable, more parallasible.Too big items.
Also it clarified who does what.
- One startup i worked (Rust).- Half year."

Антон Кот, [2022-11-30 9:39 PM]
Ты там где пишешь will it work секцию, лучше попробуй что-то типа

dzmitry lahoda, [2022-11-30 9:39 PM]
Я думал там типа персональные анекдоты написать и типа что оно улучшает каждое правило написал. Но что то лучше хорошо бы.

Антон Кот, [2022-11-30 9:40 PM]
Based on our experience atomic tasks and multiple person accounting helps to do/achieve something

dzmitry lahoda, [2022-11-30 9:40 PM]
Искать ПМ статистику в мире SAFE и Srum  - я думаю 90% процентов будет шлак.

Антон Кот, [2022-11-30 9:41 PM]
100% :)

Антон Кот, [2022-11-30 9:41 PM]
Попробуй добавить привязки к каким-то общепринятым agile техникам/принципам

dzmitry lahoda, [2022-11-30 9:42 PM]
Я думал на тот же SAFE сослаться где мен надо )))

Антон Кот, [2022-11-30 9:42 PM]
Потому что то, что ты описываешь это набор правил поверх scrum как мне кажется

dzmitry lahoda, [2022-11-30 9:42 PM]
ну да. вместо статы тупо ссылки на правила где Scrum и SAFE мне подходят

dzmitry lahoda, [2022-11-30 9:42 PM]
не

dzmitry lahoda, [2022-11-30 9:42 PM]
мне скорее пофигу Scrum

dzmitry lahoda, [2022-11-30 9:43 PM]
это скорее ортогонально процессу

Антон Кот, [2022-11-30 9:43 PM]
Типа обязательно надо пригласить вотчера, иначе такс не создастся

Антон Кот, [2022-11-30 9:44 PM]
Опять же зависит от размера команды/проекта

dzmitry lahoda, [2022-11-30 9:44 PM]
Я  думаю типа спекулятивно - создавай таск. Но если ты не согласовал ни с кем - чего ты на ним работаешь? Типа PR + reivew

Антон Кот, [2022-11-30 9:45 PM]
Не, ну, логично. У merge request есть ревьювер и у таска тоже может быть peer-надзиратель

dzmitry lahoda, [2022-11-30 9:46 PM]
В Scrum надзиратель PO или PM afaik. То есть там фильр at least one review + reivwer is PM or PO

dzmitry lahoda, [2022-11-30 9:57 PM]
вообще весь документ это мой опыт многолетний как мне работать меньше

dzmitry lahoda, [2022-11-30 9:58 PM]
типа есть
1. иерархия, централизация, синхронность
2. децентралзация, DAG, асинхрноость


dzmitry lahoda, [2022-11-30 10:19 PM]
Вообще придумал 2 отсылки на скрам и сейф.

