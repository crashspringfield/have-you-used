import Controller from '@ember/controller'

export default Controller.extend({
  actions: {
    deleteTask(id) {
      this.store.findRecord('task', id)
        .then(task => {
          task.deleteRecord()
          task.save()
        })
    }
  }
})
